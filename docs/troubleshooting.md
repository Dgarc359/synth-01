# Trials and tribulations of an audio programmer

Initial implementation had issues with combining multiple different frequencies

This issue was resolved by having each frequency track it's own independent phase
angle


#### 03-18-2025 - Envelope misbehavior

Envelope not working as expected. Holding down a key yields 0 sound, then
when a key is released, sound is heard for a moment.

It seems like the envelope is incorrectly being applied when a key is held down.
To simplify the problem and get to the root of the issue, the first step to take
is to simplify the ADSR envelope to simply just an ATTACK envelope, and validate
it's working as expected.
