/* Place the '.srt' file in "C:/User/<username>" */

gst-launch-1.0 filesrc location=subtitles.srt ! subparse ! txt. videotestsrc ! 'video/x-raw, width=1280, height=720' ! textoverlay name=txt shaded-background=true font-desc="Sans 10" halignment=right valignment=bottom line-alignment=left ! autovideosink
