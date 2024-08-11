# Shoddycast's 'Pikasprey's graveler softlock' programming contest
Because I've improved this code many times (and submitted many times, my apologies Austin) I will chronicle the timeline of my improvements here.

| Improvement | Time | Improvement | Commit |
| :---------- | :--- | :---------: | :----: |
| <details><summary>**Initial implementation**</summary>This was basically a copy of Austin's code in Rust.</details> | <details><summary>5min 40.09s</summary>340.090559094s</details> | - | [a5c41951939f57c25b74119aa9cdd3b47f46bb41](https://github.com/Ailko/Pikasprey-graveler/tree/a5c41951939f57c25b74119aa9cdd3b47f46bb41) |
| <details><summary>**Initial multithreading**</summary>This performed worse than my initial implementation as this was a very naive implementation.</details> | <details><summary>6min 7.94s</summary>367.940684051s</details> | -8.19% | [9c04743bff02232614fdda96219054b6260c7a32](https://github.com/Ailko/Pikasprey-graveler/tree/9c04743bff02232614fdda96219054b6260c7a32) |
| <details><summary>**Improve multithreading lock efficiency**</summary>Here I improved the efficiency with which I accessed shared variables by scoping a lock I should have scoped earlier. (Meaning the thread doesn't hold the other hostages up unnecessarily).</details> | <details><summary>5min 9.69s</summary>309.688722104s</details> | 15.83% | [c907f7bc4add96a4e6b52ceabd9bf2aeb61485de](https://github.com/Ailko/Pikasprey-graveler/tree/c907f7bc4add96a4e6b52ceabd9bf2aeb61485de) |
| <details><summary>**Improve overhead and clean up**</summary>Pulled the access to shared variables into functions to minimize the scope in which they're locked. Also removed some unnecessary overhead.</details> | <details><summary>4min 57.08s</summary>297.084252682s</details> | 4.07% | [fdd555adafec85445cc406ea304778f4187410c8](https://github.com/Ailko/Pikasprey-graveler/tree/fdd555adafec85445cc406ea304778f4187410c8) |
| <details><summary>**Remove shared variables**</summary>Removed the need for shared variables altogether so the threads could run uninterupted.</details> | <details><summary>43.38s</summary>43.379256549s</details> | 85.40% | [350c4bf095c1cd9f081b99a1fb1958f32d7002e2](https://github.com/Ailko/Pikasprey-graveler/tree/350c4bf095c1cd9f081b99a1fb1958f32d7002e2) |

> [!NOTE]
> For reference, the multithreaded versions always ran with 16 threads so at no point was the amount of threads changed.

This constitutes a 99.99% [^1] improvement over Austin's code (so far) ;).

[^1]: 99.9941222112478%