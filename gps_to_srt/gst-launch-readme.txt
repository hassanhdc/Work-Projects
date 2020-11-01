## Place the '.srt' file in "C:/User/<username>"

 gst-launch-1.0 -v filesrc location=subtitles.srt ! subparse ! txt.   videotestsrc ! 'video/x-raw, height=720, width=1280' ! videoconvert ! textoverlay name=txt shaded-background=yes halignment=left valignment=top line-alignment=left font-desc= 'Sans, 12' ! autovideosink
