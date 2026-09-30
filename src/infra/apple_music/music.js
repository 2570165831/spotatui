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
    // Right after play/next/previous Music can report "playing" before its
    // current track resolves (-1728 while a stream buffers). That is not a
    // failure: report no track yet and let the next poll fill it in.
    function currentTrackInfo() {
        try {
            return trackInfo(music.currentTrack);
        } catch (e) {
            return null;
        }
    }
    function snapshot() {
        const state = music.playerState();
        const track = state === 'stopped' ? null : currentTrackInfo();
        return {running: true, playing: state === 'playing', track: track,
                position: track ? music.playerPosition() : 0, volume: music.soundVolume(),
                shuffle: music.shuffleEnabled()};
    }
    // Give a just-started track up to ~2s to resolve, well inside the 8s limit.
    function awaitCurrentTrack() {
        for (let i = 0; i < 20 && currentTrackInfo() === null; ++i) delay(0.1);
    }
    // The sidebar lists the user's playlists and the Apple Music playlists
    // added to the library, which Music scripts as subscription playlists.
    function playlist(id) {
        if (id === 'library') return music.libraryPlaylists[0];
        let found = music.userPlaylists.whose({persistentID: id})();
        if (!found.length) found = music.subscriptionPlaylists.whose({persistentID: id})();
        if (!found.length) throw new Error('Playlist no longer exists in Music');
        return found[0];
    }
    // In Music's own order, without the library and anything not browsable.
    function browsablePlaylists() {
        return music.playlists().filter(function(p) {
            const cls = p.class();
            return cls === 'userPlaylist' || cls === 'subscriptionPlaylist';
        });
    }
    function page(items, offset, convert) {
        const total = items.length;
        const start = Math.min(Number(offset), total);
        const next = Math.min(start + 100, total);
        const rows = [];
        // Music's search can return entries that no longer resolve (every
        // property fails with -1728); skip them rather than fail the page.
        for (let i = start; i < next; ++i) {
            try { rows.push(convert(items[i])); } catch (e) {}
        }
        return {items: rows, offset: start, total: total, next: next};
    }
    switch (op) {
    case 'playlists':
        return JSON.stringify(page(browsablePlaylists(), argv[1], function(p) {
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
        awaitCurrentTrack();
        break;
    }
    case 'resume': music.play(); break;
    case 'pause': music.pause(); break;
    case 'next': music.nextTrack(); awaitCurrentTrack(); break;
    case 'previous': music.previousTrack(); awaitCurrentTrack(); break;
    case 'seek': music.playerPosition = Math.min(Number(argv[1]) / 1000, music.currentTrack.duration()); break;
    case 'volume': music.soundVolume = Number(argv[1]); break;
    case 'snapshot': break;
    default: throw new Error('Unknown Music operation');
    }
    return JSON.stringify(snapshot());
}
