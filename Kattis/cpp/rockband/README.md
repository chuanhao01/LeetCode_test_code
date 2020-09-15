Every day after school, you and your friends get together and play in a band. Over the past couple of months, the band has been rehearsing a large number of songs. Now it’s time to go out and perform in front of a crowd for the first time. In order to do so, a set list for the concert needs to be determined.

As it turns out, every band member has a different taste in music. (Who would have thought?) Everybody is very picky: a band member doesn’t want to play any particular song X unless he also gets to play all songs he likes better than song X. This holds for every band member and for every song X. Furthermore, obviously at least one song should be performed.

The organisers of the concert do not want you to play too many songs, so a selection needs to be made that is as small as possible. As the unofficial leader of the band, you have taken it upon yourself to find a minimum length set list that meets the requirements.

Input
The first line contains two integers M and S, satisfying M≥1 and S≥1 as well as M⋅S≤106. These denote the total number of band members and the number of songs, respectively.

The following M lines each contain S integers per line, where the i-th line denotes the preference list of the i-th band member, starting with his favourite song and ending with his least favourite song. The songs are numbered 1 through S.

No two band members have the exact same preference lists.

Output
Output the smallest possible set list, using the following format:

One line with an integer L: the length of the smallest possible set list.

One line with L space-separated integers, denoting a sorted list of the songs to be played.

