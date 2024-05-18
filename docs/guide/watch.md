# Slides rebuilding

Nelsie can automatically rebuild slides when the source Python file is changed or when images used in slides are
changed.
It is started by the following command (lets assume that our slides are defined in `slides.py`).

```commandline
$ python3 -m nelsie watch slides.py
```

Nelsie builds the slides and starts to watch `slides.py` and used images and rebuilds the slides when relevant files are
changed.
Note that if the first build fails, the watch is not started.