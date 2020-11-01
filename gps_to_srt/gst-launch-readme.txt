Store the '.srt' file in "C:/User/<username>"

gst-launch-1.0 -v filesrc location=subtitles.srt ! subparse ! txt.   videotestsrc ! textoverlay name=txt shaded-background=yes halignment=left valignment=top line-alignment=left ! autovideosink