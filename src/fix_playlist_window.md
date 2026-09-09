# Fix Playlist Window

The `getPlaylistWindow` function should return a section of a playlist.

It receives:

- `songs`: the complete playlist
- `start`: the index of the first song to include
- `count`: the maximum number of songs to include

The returned array must preserve the original order. If fewer than `count` songs remain, return all remaining songs. Do not change the original playlist.

## Example

```js
getPlaylistWindow(["Intro", "Storm", "Sunrise", "Finale"], 1, 2);
// ["Storm", "Sunrise"]
```

Fix the function so it returns the correct playlist window.
