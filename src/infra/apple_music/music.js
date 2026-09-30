// Constant JXA program, executed by osascript. All inputs are argv values.
// Never activate, reveal, select, or construct/evaluate caller-supplied code.
function run(argv) {
    const music = Application('com.apple.Music');
    const op = argv[0];
    const stopped = {running: false, playing: false, track: null, position: 0, volume: 0};
    if (!music.running()) {
        return JSON.stringify(op === 'snapshot' || op === 'pause' ? stopped : {not_running: true});
    }
    function trackInfo(track) {
        return {id: track.persistentID(), name: track.name(), artist: track.artist(),
                album: track.album(), duration: track.duration()};
    }
    function snapshot() {
        const state = music.playerState();
        const track = state === 'stopped' ? null : trackInfo(music.currentTrack);
        return {running: true, playing: state === 'playing', track: track,
                position: track ? music.playerPosition() : 0, volume: music.soundVolume()};
    }
    function playlist(id) {
        if (id === 'library') return music.libraryPlaylists[0];
        const found = music.userPlaylists.whose({persistentID: id})();
        if (!found.length) throw new Error('Playlist no longer exists in Music');
        return found[0];
    }
    function page(items, offset, convert) {
        const total = items.length;
        const start = Math.min(Number(offset), total);
        const rows = [];
        for (let i = start; i < Math.min(start + 100, total); ++i) rows.push(convert(items[i]));
        return {items: rows, offset: start, total: total};
    }
    switch (op) {
    case 'playlists':
        return JSON.stringify(page(music.userPlaylists(), argv[1], function(p) {
            return {id: p.persistentID(), name: p.name()};
        }));
    case 'tracks':
        return JSON.stringify(page(playlist(argv[1]).tracks(), argv[2], trackInfo));
    case 'search':
        return JSON.stringify(page(music.search(music.libraryPlaylists[0], {for: argv[1], only: 'all'}) || [], argv[2], trackInfo));
    case 'play': {
        const context = argv[1] === 'track' ? music.libraryPlaylists[0] : playlist(argv[2]);
        const id = argv[1] === 'track' ? argv[2] : argv[3];
        let selected;
        if (id) {
            const found = context.tracks.whose({persistentID: id})();
            if (!found.length) throw new Error('Track no longer exists in Music');
            selected = found[0];
        } else {
            if (Number(argv[4]) >= context.tracks.length) throw new Error('Playlist is empty or offset is out of range');
            selected = context.tracks[Number(argv[4])];
        }
        // Playing the track specifier in its playlist preserves Music's own
        // Next/Previous order. No spotatui cross-source queue is constructed.
        music.play(selected);
        break;
    }
    case 'resume': music.play(); break;
    case 'pause': music.pause(); break;
    case 'next': music.nextTrack(); break;
    case 'previous': music.previousTrack(); break;
    case 'seek': music.playerPosition = Math.min(Number(argv[1]) / 1000, music.currentTrack.duration()); break;
    case 'volume': music.soundVolume = Number(argv[1]); break;
    case 'snapshot': break;
    default: throw new Error('Unknown Music operation');
    }
    return JSON.stringify(snapshot());
}
