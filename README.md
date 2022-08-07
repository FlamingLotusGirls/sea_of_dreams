# Sea of dreams
Things related to the sea of dreams

Note: This is currently just a copy of the serenity code. The changes to ensure it's not distributed and the updated config are NOT checked in.

## Audio and Sound

Please see the sound directory.

Generally, there is a daemon running on a Raspberry pi.
These are written in C, have a simple HTTP server to receive REST calls, and are written to 
PulseAudio, an audio interface which allows easy manipulation of multiple outputs, and volumes and modules
which are manipulated via code. While there are interfaces in higher level languages, the "zero copy"
nature of PulseAudio is best done through C, and C is a language I like.

Due to the fact that it's hard to store audio files, we've created a google drive folder which has the
actual files. That directory should be copied directly into a SerenityAudio server.

There is then another server on the "master" which is driven by the admin UI, and buttons, and knows about the
PIs throughout the sculpture.

 
