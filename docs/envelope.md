

Initial envelope implementation:
```
      _
     / \
    /   2
   /     \__3__
  /            \
 1              4
/                \
```
Legend:
1 - Attack
2 - Decay
3 - Sustain
4 - Release

Envelopes currently are represented by a struct containing an enum telling us what the current envelope state is

each envelope state has its own configuration that is unique to itself


