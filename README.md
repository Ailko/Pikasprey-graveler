# Shoddycast's 'Pikasprey's graveler softlock' programming contest
Because I've improved this code many times (and submitted many times, my apologies Austin) I will chronicle the timeline of my improvements here.

| Improvement | Time | Commit |
| :---------- | :--- | :----: |
| <details><summary>**Initial implementation**</summary>This was basically a copy of Austin's code in Rust.</details> | 5min 40.09s<details>340.090559094s</details> | a5c41951939f57c25b74119aa9cdd3b47f46bb41 |
| <details><summary>**Initial multithreading**</summary>This performed worse than my initial implementation as this was a very naive implementation.</details> | 6min 7.94s<details>367.940684051s</details> | 9c04743bff02232614fdda96219054b6260c7a32 |
| <details><summary>**Improve multithreading lock efficiency**</summary>Here I improved the efficiency with which I accessed shared variables by scoping a lock I should have scoped earlier. (Meaning the thread doesn't hold the other hostages up unnecessarily).</details> | 5min 9.69s<details>309.688722104s</details> | c907f7bc4add96a4e6b52ceabd9bf2aeb61485de |
| <details><summary>**Improve overhead and clean up**</summary>Pulled the access to shared variables into functions to minimize the scope in which they're locked. Also removed some unnecessary overhead.</details> | 4min 57.08s<details>297.084252682s</details> | fdd555adafec85445cc406ea304778f4187410c8 |
| <details><summary>**Remove shared variables**</summary>Removed the need for shared variables altogether so the threads could run uninterupted.</details> | 43.38s<details>43.379256549s</details> | 350c4bf095c1cd9f081b99a1fb1958f32d7002e2 |

> [!NOTE]
> (For reference) the multithreaded versions always ran with 16 threads so at no point was the amount of threads changed.

This constitutes a [99.99%](## "99.9941222112478%") improvement over Austin's code (so far).

