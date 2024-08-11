# Shoddycast's 'Pikasprey's graveler softlock' programming contest
Because I've improved this code many times (and submitted many times, my apologies Austin) I will chronicle the timeline of my improvements here.

| Improvement | Time |
| :---------- | :--- |
| Initial implementation<br><sub><sup>This was basically a copy of Austin's code in Rust.</sup></sub> | [5min 40.09s](## "340.090559094s") |
| Initial multithreading<br><sub><sup>This performed worse than my initial implementation as this was a very naive implementation.</sup></sub> | [6min 7.94s](## "367.940684051s") |
| Improve multithreading lock efficiency<br><sub><sup>Here I improved the efficiency with which I accessed shared variables by scoping a lock I should have scoped earlier. (Meaning the thread doesn't hold the other hostages up unnecessarily).</sup></sub> | [5min 9.69s](## "309.688722104s") |
| Improve overhead and clean up<br><sub><sup>Pulled the access to shared variables into functions to minimize the scope in which they're locked. Also removed some unnecessary overhead.</sup></sub> | [4min 57.08s](## "297.084252682s") |
| Remove shared variables<br><sub><sup>Removed the need for shared variables altogether so the threads could run uninterupted.</sup></sub> | [43.38s](## "43.379256549s") |

<sup><sup>(For reference) the multithreaded versions always ran with 16 threads so at no point was the amount of threads changed.</sup></sup>

This constitutes a [99.99%](## "99.9941222112478%") improvement over Austin's code (so far).

