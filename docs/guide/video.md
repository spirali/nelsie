# Videos

Nelsie is able to embedd video into PDF slides.
If and how video is played and how looks the control panel depends on your PDF viewer.

When slide is shown, it shows a cover image and video is played when 
you click on the cover image.

```python
@deck.slide()
def video_demo(slide):
    slide.video(
        "myfile.mp4", cover_image="cover_image.jpg", width=300, height=300)
```

By default, no control panel is shown; you can change it by setting: `show_controls=True`:

```python
@deck.slide()
def video_demo(slide):
    slide.video(
        "myfile.mp4", cover_image="cover_image.jpg",
        width=300, height=300, show_controls=True)
```

!!! warning "Video box size"

    In the current version, Nelsie does not detect size of the video.
    You have to manually set the size of video box via `width` and `height` attributes.


!!! warning "Supported formats for cover images"  
    Nelsie supports only PNG and JPEG images as cover images.


Note that video embedding works only for PDF output; PNG/SVG output shows
only cover image if defined; otherwise, the video box just reserves layout size but nothing is shown.

