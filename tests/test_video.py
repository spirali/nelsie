from conftest import ASSETS_DIR
from testutils import check
import os
import pytest

# Temporarily disabled
# @check()
# def test_embed_video_with_cover_image(deck):
#     slide = deck.new_slide(width=400, height=400)
#     slide.video(os.path.join(ASSETS_DIR, "video.mp4"), cover_image="testimg.jpeg", width=350, height=280)
#
#
# @check()
# def test_embed_video_no_cover_image(deck):
#     slide = deck.new_slide(width=400, height=400)
#     slide.video(os.path.join(ASSETS_DIR, "video.mp4"), width=350, height=280)
#
#
# def test_embed_video_invalid_files(deck):
#     slide = deck.new_slide(width=400, height=400)
#     with pytest.raises(Exception, match="cover image: "):
#         slide.video(os.path.join(ASSETS_DIR, "video.mp4"), cover_image="non-existent-image.jpeg", width=350, height=280)
#     with pytest.raises(Exception, match="cover image: Invalid format \\(only formats png and jpeg are supported\\)"):
#         slide.video(os.path.join(ASSETS_DIR, "video.mp4"), cover_image="test.svg", width=350, height=280)
#     with pytest.raises(Exception, match="Video file does not exist:"):
#         slide.video(
#             os.path.join(ASSETS_DIR, "non-existent-video.mp4"), cover_image="testimg.jpeg", width=350, height=280
#         )
