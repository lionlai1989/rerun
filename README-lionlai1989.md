# My rerun guide

## install

```bash
python3 -m venv venv_rerun
source venv_rerun/bin/activate
python3 -m pip install rerun-sdk==0.22.0

# check out tag 0.22.0
```

### run structure from motion example
- "numpy<2" in "examples/python/structure_from_motion/pyproject.toml"

```bash
python3 -m pip install -e examples/python/structure_from_motion

touch /Users/lionlai/rerun/examples/python/structure_from_motion/dataset/barbershop/colmap/sparse/0/points3D.txt
python3 -m structure_from_motion --dataset barbershop

python3 -m structure_from_motion --dataset colmap_rusty_car


python3 -m structure_from_motion --dataset homeeai

```
