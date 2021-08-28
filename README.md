# sort-for-short-array

## bench

env: macOS

### T=i32 N=5

| target | time |
|:------:|-----:|
| `[T]::sort` | 11151 ps |
| `sorting_network::sort5` | 595 ps |

## Sorting Network

* [wikipedia](https://ja.wikipedia.org/wiki/%E3%82%BD%E3%83%BC%E3%83%86%E3%82%A3%E3%83%B3%E3%82%B0%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF)
* [generate](https://www.wolframcloud.com/objects/demonstrations/SortingNetworks-source.nb)
