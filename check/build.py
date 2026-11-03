import re, pathlib, subprocess, sys
here = pathlib.Path(__file__).parent
b = here.parent / 'mixins' / 'chat_animation'
t = (here / 'template.rs').read_text().replace('//ANIM//', (b / 'anim.rs').read_text())
for f in sorted(b.glob('*.rs')):
    if f.name == 'anim.rs':
        continue
    name = f.stem
    src = re.sub(r'\bfn chat_animation_\w+\(', f'fn __{name}(', f.read_text(), count=1)
    t = t.replace(f'//{name.upper()}//', src)
(here / 'check.rs').write_text(t)
r = subprocess.run(['rustc', '--edition', '2024', '--crate-type', 'lib', '--crate-name', 'horizon_client', 'check.rs', '-o', '/tmp/ca_check.rlib'], cwd=here, capture_output=True, text=True)
print(r.stderr[-8000:])
if r.returncode or '--test' not in sys.argv:
    sys.exit(r.returncode)
(here / 'check_test.rs').write_text(t + '\n#[cfg(test)]\n#[path = "' + str((here.parent / 'test' / 'tests.rs').resolve()) + '"]\nmod tests;\n')
r = subprocess.run(['rustc', '--edition', '2024', '--test', '--crate-name', 'horizon_client', 'check_test.rs', '-o', '/tmp/ca_tests'], cwd=here, capture_output=True, text=True)
print(r.stderr[-8000:])
if r.returncode:
    sys.exit(r.returncode)
sys.exit(subprocess.run(['/tmp/ca_tests', '--test-threads=1']).returncode)
