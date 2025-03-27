#include <math.h>

#define outpin 2 // audio out to speaker

void setup() {
  // https://docs.arduino.cc/language-reference/en/functions/communication/serial/begin/
  // we're setting our data rate in bits per second (baud)
  // to 9600 b/s
  Serial.begin(9600);

}

bool flip_note = false;

void loop() {
  int note_eval, note_duration;
  note_duration = 1000;
  if (flip_note) {
    note_eval = 261; // MIDDLE C
  } else {
    // switch to a completely different not to validate sound output is correct
    note_eval = 1760; // A6
  }
  freqout(note_eval, note_duration);
  delay(10);
  flip_note = ! flip_note;
}

void freqout(int freq, int t)  // freq in hz, t in ms
{
  int hperiod;                               //calculate 1/2 period in us
  long cycles, i;
  pinMode(outpin, OUTPUT);                   // turn on output pin

  hperiod = (500000 / freq) - 7;             // subtract 7 us to make up for digitalWrite overhead
  cycles = ((long)freq * (long)t) / 1000;    // calculate cycles

  for (i=0; i<= cycles; i++){              // play note for t ms
    digitalWrite(outpin, HIGH);
    delayMicroseconds(hperiod);
    digitalWrite(outpin, LOW);
    delayMicroseconds(hperiod - 1);     // - 1 to make up for digitaWrite overhead
  }

  pinMode(outpin, INPUT);                // shut off pin to avoid noise from other operations
}

