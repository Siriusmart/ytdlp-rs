use super::Builder;

/// General Options
impl Builder {
    /// Print this help text and exit
    pub fn help(&mut self) -> &mut Self {
        self.command_mut().arg("--help");
        self
    }

    /// Print program version and exit
    pub fn version(&mut self) -> &mut Self {
        self.command_mut().arg("--version");
        self
    }

    /// Check if updates are available. You installed yt-dlp from a manual build or with a package manager; Use that to update
    pub fn update(&mut self) -> &mut Self {
        self.command_mut().arg("--update");
        self
    }

    /// Do not check for updates (default)
    pub fn no_update(&mut self) -> &mut Self {
        self.command_mut().arg("--no-update");
        self
    }

    /// Upgrade/downgrade to a specific version. CHANNEL can be a repository as well. CHANNEL and TAG default to "stable" and "latest" respectively if omitted; See "UPDATE" for details. Supported channels: stable, nightly, master
    pub fn update_to(&mut self, channeltag: &str) -> &mut Self {
        self.command_mut().arg("--update-to");
        self.command_mut().arg(channeltag);
        self
    }

    /// Ignore download and postprocessing errors. The download will be considered successful even if the postprocessing fails
    pub fn ignore_errors(&mut self) -> &mut Self {
        self.command_mut().arg("--ignore-errors");
        self
    }

    /// Continue with next video on download errors; e.g. to skip unavailable videos in a playlist (default)
    pub fn no_abort_on_error(&mut self) -> &mut Self {
        self.command_mut().arg("--no-abort-on-error");
        self
    }

    /// Abort downloading of further videos if an error occurs (Alias: --no-ignore-errors)
    pub fn abort_on_error(&mut self) -> &mut Self {
        self.command_mut().arg("--abort-on-error");
        self
    }

    /// Display the current user-agent and exit
    pub fn dump_user_agent(&mut self) -> &mut Self {
        self.command_mut().arg("--dump-user-agent");
        self
    }

    /// List all supported extractors and exit
    pub fn list_extractors(&mut self) -> &mut Self {
        self.command_mut().arg("--list-extractors");
        self
    }

    /// Output descriptions of all supported extractors and exit
    pub fn extractor_descriptions(&mut self) -> &mut Self {
        self.command_mut().arg("--extractor-descriptions");
        self
    }

    /// Extractor names to use separated by commas. You can also use regexes, "all", "default" and "end" (end URL matching); e.g. --ies "holodex.*,end,youtube". Prefix the name with a "-" to exclude it, e.g. --ies default,-generic. Use --list-extractors for a list of extractor names. (Alias: --ies)
    pub fn use_extractors(&mut self, names: &str) -> &mut Self {
        self.command_mut().arg("--use-extractors");
        self.command_mut().arg(names);
        self
    }

    /// Use this prefix for unqualified URLs. E.g. "gvsearch2:python" downloads two videos from google videos for the search term "python". Use the value "auto" to let yt-dlp guess ("auto_warning" to emit a warning when guessing). "error" just throws an error. The default value "fixup_error" repairs broken URLs, but emits an error if this is not possible instead of searching
    pub fn default_search(&mut self, prefix: &str) -> &mut Self {
        self.command_mut().arg("--default-search");
        self.command_mut().arg(prefix);
        self
    }

    /// Don't load any more configuration files except those given to --config-locations. For backward compatibility, if this option is found inside the system configuration file, the user configuration is not loaded. (Alias: --no-config)
    pub fn ignore_config(&mut self) -> &mut Self {
        self.command_mut().arg("--ignore-config");
        self
    }

    /// Do not load any custom configuration files (default). When given inside a configuration file, ignore all previous --config-locations defined in the current file
    pub fn no_config_locations(&mut self) -> &mut Self {
        self.command_mut().arg("--no-config-locations");
        self
    }

    /// Location of the main configuration file; either the path to the config or its containing directory ("-" for stdin). Can be used multiple times and inside other configuration files
    pub fn config_locations(&mut self, path: &str) -> &mut Self {
        self.command_mut().arg("--config-locations");
        self.command_mut().arg(path);
        self
    }

    /// Do not extract the videos of a playlist, only list them
    pub fn flat_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--flat-playlist");
        self
    }

    /// Fully extract the videos of a playlist (default)
    pub fn no_flat_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--no-flat-playlist");
        self
    }

    /// Download livestreams from the start. Currently only supported for YouTube (Experimental)
    pub fn live_from_start(&mut self) -> &mut Self {
        self.command_mut().arg("--live-from-start");
        self
    }

    /// Download livestreams from the current time (default)
    pub fn no_live_from_start(&mut self) -> &mut Self {
        self.command_mut().arg("--no-live-from-start");
        self
    }

    /// Wait for scheduled streams to become available. Pass the minimum number of seconds (or range) to wait between retries
    pub fn wait_for_video(&mut self, minmax: &str) -> &mut Self {
        self.command_mut().arg("--wait-for-video");
        self.command_mut().arg(minmax);
        self
    }

    /// Do not wait for scheduled streams (default)
    pub fn no_wait_for_video(&mut self) -> &mut Self {
        self.command_mut().arg("--no-wait-for-video");
        self
    }

    /// Mark videos watched (even with --simulate)
    pub fn mark_watched(&mut self) -> &mut Self {
        self.command_mut().arg("--mark-watched");
        self
    }

    /// Do not mark videos watched (default)
    pub fn no_mark_watched(&mut self) -> &mut Self {
        self.command_mut().arg("--no-mark-watched");
        self
    }

    /// Whether to emit color codes in output, optionally prefixed by the STREAM (stdout or stderr) to apply the setting to. Can be one of "always", "auto" (default), "never", or "no_color" (use non color terminal sequences). Use "auto-tty" or "no_color-tty" to decide based on terminal support only. Can be used multiple times
    pub fn color(&mut self, streampolicy: &str) -> &mut Self {
        self.command_mut().arg("--color");
        self.command_mut().arg(streampolicy);
        self
    }

    /// Options that can help keep compatibility with youtube-dl or youtube-dlc configurations by reverting some of the changes made in yt-dlp. See "Differences in default behavior" for details an alias starts with a dash "-", it is prefixed with "--". Arguments are parsed according to the Python string formatting mini-language. E.g. --alias get-audio,-X "-S=aext:{0},abr -x --audio-format {0}" creates options "--get-audio" and "-X" that takes an argument (ARG0) and expands to "-S=aext:ARG0,abr -x --audio-format ARG0". All defined aliases are listed in the --help output. Alias options can trigger more aliases; so be careful to avoid defining recursive options. As a safety measure, each alias may be triggered a maximum of 100 times. This option can be used multiple times
    pub fn compat_options(&mut self, opts: &str) -> &mut Self {
        self.command_mut().arg("--compat-options");
        self.command_mut().arg(opts);
        self
    }
}

/// Network Options
impl Builder {
    /// Use the specified HTTP/HTTPS/SOCKS proxy. To enable SOCKS proxy, specify a proper scheme, e.g. socks5://user:pass@127.0.0.1:1080/. Pass in an empty string (--proxy "") for direct connection
    pub fn proxy(&mut self, url: &str) -> &mut Self {
        self.command_mut().arg("--proxy");
        self.command_mut().arg(url);
        self
    }

    /// Time to wait before giving up, in seconds
    pub fn socket_timeout(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--socket-timeout");
        self.command_mut().arg(seconds);
        self
    }

    /// Client-side IP address to bind to
    pub fn source_address(&mut self, ip: &str) -> &mut Self {
        self.command_mut().arg("--source-address");
        self.command_mut().arg(ip);
        self
    }

    /// Client to impersonate for requests. E.g. chrome, chrome-110, chrome:windows-10. Pass --impersonate="" to impersonate any client. Note that forcing impersonation for all requests may have a detrimental impact on download speed and stability
    pub fn impersonate(&mut self, clientos: &str) -> &mut Self {
        self.command_mut().arg("--impersonate");
        self.command_mut().arg(clientos);
        self
    }

    /// List available clients to impersonate.
    pub fn list_impersonate_targets(&mut self) -> &mut Self {
        self.command_mut().arg("--list-impersonate-targets");
        self
    }

    /// Make all connections via IPv4
    pub fn force_ipv4(&mut self) -> &mut Self {
        self.command_mut().arg("--force-ipv4");
        self
    }

    /// Make all connections via IPv6
    pub fn force_ipv6(&mut self) -> &mut Self {
        self.command_mut().arg("--force-ipv6");
        self
    }

    /// Enable file:// URLs. This is disabled by default for security reasons.
    pub fn enable_file_urls(&mut self) -> &mut Self {
        self.command_mut().arg("--enable-file-urls");
        self
    }

    /// Use this proxy to verify the IP address for some geo-restricted sites. The default proxy specified by --proxy (or none, if the option is not present) is used for the actual downloading
    pub fn geo_verification_proxy(&mut self, url: &str) -> &mut Self {
        self.command_mut().arg("--geo-verification-proxy");
        self.command_mut().arg(url);
        self
    }

    /// How to fake X-Forwarded-For HTTP header to try bypassing geographic restriction. One of "default" (only when known to be useful), "never", an IP block in CIDR notation, or a two-letter ISO 3166-2 country code
    pub fn xff(&mut self, value: &str) -> &mut Self {
        self.command_mut().arg("--xff");
        self.command_mut().arg(value);
        self
    }

    /// Comma separated playlist_index of the items to download. You can specify a range using "[START]:[STOP][:STEP]". For backward compatibility, START-STOP is also supported. Use negative indices to count from the right and negative STEP to download in reverse order. E.g. "-I 1:3,7,-5::2" used on a playlist of size 15 will download the items at index 1,2,3,7,11,13,15
    pub fn playlist_items(&mut self, itemspec: &str) -> &mut Self {
        self.command_mut().arg("--playlist-items");
        self.command_mut().arg(itemspec);
        self
    }

    /// Abort download if filesize is smaller than SIZE, e.g. 50k or 44.6M
    pub fn min_filesize(&mut self, size: &str) -> &mut Self {
        self.command_mut().arg("--min-filesize");
        self.command_mut().arg(size);
        self
    }

    /// Abort download if filesize is larger than SIZE, e.g. 50k or 44.6M
    pub fn max_filesize(&mut self, size: &str) -> &mut Self {
        self.command_mut().arg("--max-filesize");
        self.command_mut().arg(size);
        self
    }

    /// Download only videos uploaded on this date. The date can be "YYYYMMDD" or in the format [now|today|yesterday][- N[day|week|month|year]]. E.g. "--date today-2weeks" downloads only videos uploaded on the same day two weeks ago
    pub fn date(&mut self, date: &str) -> &mut Self {
        self.command_mut().arg("--date");
        self.command_mut().arg(date);
        self
    }

    /// Download only videos uploaded on or before this date. The date formats accepted is the same as --date
    pub fn datebefore(&mut self, date: &str) -> &mut Self {
        self.command_mut().arg("--datebefore");
        self.command_mut().arg(date);
        self
    }

    /// Download only videos uploaded on or after this date. The date formats accepted is the same as --date
    pub fn dateafter(&mut self, date: &str) -> &mut Self {
        self.command_mut().arg("--dateafter");
        self.command_mut().arg(date);
        self
    }

    /// Generic video filter. Any "OUTPUT TEMPLATE" field can be compared with a number or a string using the operators defined in "Filtering Formats". You can also simply specify a field to match if the field is present, use "!field" to check if the field is not present, and "&" to check multiple conditions. Use a "\" to escape "&" or quotes if needed. If used multiple times, the filter matches if at least one of the conditions is met. E.g. --match-filter !is_live --match-filter "like_count>?100 & description~='(?i)\bcats \& dogs\b'" matches only videos that are not live OR those that have a like count more than 100 (or the like field is not available) and also has a description that contains the phrase "cats & dogs" (caseless). Use "--match-filter -" to interactively ask whether to download each video
    pub fn match_filters(&mut self, filter: &str) -> &mut Self {
        self.command_mut().arg("--match-filters");
        self.command_mut().arg(filter);
        self
    }

    /// Do not use any --match-filter (default)
    pub fn no_match_filters(&mut self) -> &mut Self {
        self.command_mut().arg("--no-match-filters");
        self
    }

    /// Same as "--match-filters" but stops the download process when a video is rejected
    pub fn break_match_filters(&mut self, filter: &str) -> &mut Self {
        self.command_mut().arg("--break-match-filters");
        self.command_mut().arg(filter);
        self
    }

    /// Do not use any --break-match-filters (default)
    pub fn no_break_match_filters(&mut self) -> &mut Self {
        self.command_mut().arg("--no-break-match-filters");
        self
    }

    /// Download only the video, if the URL refers to a video and a playlist
    pub fn no_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--no-playlist");
        self
    }

    /// Download the playlist, if the URL refers to a video and a playlist
    pub fn yes_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--yes-playlist");
        self
    }

    /// Download only videos suitable for the given age
    pub fn age_limit(&mut self, years: &str) -> &mut Self {
        self.command_mut().arg("--age-limit");
        self.command_mut().arg(years);
        self
    }

    /// Download only videos not listed in the archive file. Record the IDs of all downloaded videos in it
    pub fn download_archive(&mut self, file: &str) -> &mut Self {
        self.command_mut().arg("--download-archive");
        self.command_mut().arg(file);
        self
    }

    /// Do not use archive file (default)
    pub fn no_download_archive(&mut self) -> &mut Self {
        self.command_mut().arg("--no-download-archive");
        self
    }

    /// Abort after downloading NUMBER files
    pub fn max_downloads(&mut self, number: &str) -> &mut Self {
        self.command_mut().arg("--max-downloads");
        self.command_mut().arg(number);
        self
    }

    /// Stop the download process when encountering a file that is in the archive
    pub fn break_on_existing(&mut self) -> &mut Self {
        self.command_mut().arg("--break-on-existing");
        self
    }

    /// Do not stop the download process when encountering a file that is in the archive (default)
    pub fn no_break_on_existing(&mut self) -> &mut Self {
        self.command_mut().arg("--no-break-on-existing");
        self
    }

    /// Alters --max-downloads, --break-on-existing, --break-match-filter, and autonumber to reset per input URL
    pub fn break_per_input(&mut self) -> &mut Self {
        self.command_mut().arg("--break-per-input");
        self
    }

    /// --break-on-existing and similar options terminates the entire download queue
    pub fn no_break_per_input(&mut self) -> &mut Self {
        self.command_mut().arg("--no-break-per-input");
        self
    }

    /// Number of allowed failures until the rest of the playlist is skipped
    pub fn skip_playlist_after_errors(&mut self, n: &str) -> &mut Self {
        self.command_mut().arg("--skip-playlist-after-errors");
        self.command_mut().arg(n);
        self
    }
}

/// Download Options
impl Builder {
    /// Number of fragments of a dash/hlsnative video that should be downloaded concurrently (default is 1)
    pub fn concurrent_fragments(&mut self, n: &str) -> &mut Self {
        self.command_mut().arg("--concurrent-fragments");
        self.command_mut().arg(n);
        self
    }

    /// Maximum download rate in bytes per second, e.g. 50K or 4.2M
    pub fn limit_rate(&mut self, rate: &str) -> &mut Self {
        self.command_mut().arg("--limit-rate");
        self.command_mut().arg(rate);
        self
    }

    /// Minimum download rate in bytes per second below which throttling is assumed and the video data is re-extracted, e.g. 100K
    pub fn throttled_rate(&mut self, rate: &str) -> &mut Self {
        self.command_mut().arg("--throttled-rate");
        self.command_mut().arg(rate);
        self
    }

    /// Number of retries (default is 10), or "infinite"
    pub fn retries(&mut self, retries: &str) -> &mut Self {
        self.command_mut().arg("--retries");
        self.command_mut().arg(retries);
        self
    }

    /// Number of times to retry on file access error (default is 3), or "infinite"
    pub fn file_access_retries(&mut self, retries: &str) -> &mut Self {
        self.command_mut().arg("--file-access-retries");
        self.command_mut().arg(retries);
        self
    }

    /// Number of retries for a fragment (default is 10), or "infinite" (DASH, hlsnative and ISM)
    pub fn fragment_retries(&mut self, retries: &str) -> &mut Self {
        self.command_mut().arg("--fragment-retries");
        self.command_mut().arg(retries);
        self
    }

    /// Time to sleep between retries in seconds (optionally) prefixed by the type of retry (http (default), fragment, file_access, extractor) to apply the sleep to. EXPR can be a number, linear=START[:END[:STEP=1]] or exp=START[:END[:BASE=2]]. This option can be used multiple times to set the sleep for the different retry types, e.g. --retry-sleep linear=1::2 --retry-sleep fragment:exp=1:20
    pub fn retry_sleep(&mut self, typeexpr: &str) -> &mut Self {
        self.command_mut().arg("--retry-sleep");
        self.command_mut().arg(typeexpr);
        self
    }

    /// Skip unavailable fragments for DASH, hlsnative and ISM downloads (default) (Alias: --no-abort-on-unavailable-fragments)
    pub fn skip_unavailable_fragments(&mut self) -> &mut Self {
        self.command_mut().arg("--skip-unavailable-fragments");
        self
    }

    ///  Abort download if a fragment is unavailable (Alias: --no-skip-unavailable-fragments)
    pub fn abort_on_unavailable_fragments(&mut self) -> &mut Self {
        self.command_mut().arg("--abort-on-unavailable-fragments");
        self
    }

    /// Keep downloaded fragments on disk after downloading is finished
    pub fn keep_fragments(&mut self) -> &mut Self {
        self.command_mut().arg("--keep-fragments");
        self
    }

    /// Delete downloaded fragments after downloading is finished (default)
    pub fn no_keep_fragments(&mut self) -> &mut Self {
        self.command_mut().arg("--no-keep-fragments");
        self
    }

    /// Size of download buffer, e.g. 1024 or 16K (default is 1024)
    pub fn buffer_size(&mut self, size: &str) -> &mut Self {
        self.command_mut().arg("--buffer-size");
        self.command_mut().arg(size);
        self
    }

    /// The buffer size is automatically resized from an initial value of --buffer-size (default)
    pub fn resize_buffer(&mut self) -> &mut Self {
        self.command_mut().arg("--resize-buffer");
        self
    }

    /// Do not automatically adjust the buffer size
    pub fn no_resize_buffer(&mut self) -> &mut Self {
        self.command_mut().arg("--no-resize-buffer");
        self
    }

    /// Size of a chunk for chunk-based HTTP downloading, e.g. 10485760 or 10M (default is disabled). May be useful for bypassing bandwidth throttling imposed by a webserver (experimental)
    pub fn http_chunk_size(&mut self, size: &str) -> &mut Self {
        self.command_mut().arg("--http-chunk-size");
        self.command_mut().arg(size);
        self
    }

    /// Download playlist videos in random order
    pub fn playlist_random(&mut self) -> &mut Self {
        self.command_mut().arg("--playlist-random");
        self
    }

    /// Process entries in the playlist as they are received. This disables n_entries, --playlist-random and --playlist-reverse
    pub fn lazy_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--lazy-playlist");
        self
    }

    /// Process videos in the playlist only after the entire playlist is parsed (default)
    pub fn no_lazy_playlist(&mut self) -> &mut Self {
        self.command_mut().arg("--no-lazy-playlist");
        self
    }

    /// Set file xattribute ytdl.filesize with expected file size
    pub fn xattr_set_filesize(&mut self) -> &mut Self {
        self.command_mut().arg("--xattr-set-filesize");
        self
    }

    /// Use the mpegts container for HLS videos; allowing some players to play the video while downloading, and reducing the chance of file corruption if download is interrupted. This is enabled by default for live streams
    pub fn hls_use_mpegts(&mut self) -> &mut Self {
        self.command_mut().arg("--hls-use-mpegts");
        self
    }

    /// Do not use the mpegts container for HLS videos. This is default when not downloading live streams
    pub fn no_hls_use_mpegts(&mut self) -> &mut Self {
        self.command_mut().arg("--no-hls-use-mpegts");
        self
    }

    /// Download only chapters that match the regular expression. A "*" prefix denotes time-range instead of chapter. Negative timestamps are calculated from the end. "*from-url" can be used to download between the "start_time" and "end_time" extracted from the URL. Needs ffmpeg. This option can be used multiple times to download multiple sections, e.g. --download-sections "*10:15-inf" --download-sections "intro"
    pub fn download_sections(&mut self, regex: &str) -> &mut Self {
        self.command_mut().arg("--download-sections");
        self.command_mut().arg(regex);
        self
    }

    /// Name or path of the external downloader to use (optionally) prefixed by the protocols (http, ftp, m3u8, dash, rstp, rtmp, mms) to use it for. Currently supports native, aria2c, avconv, axel, curl, ffmpeg, httpie, wget. You can use this option multiple times to set different downloaders for different protocols. E.g. --downloader aria2c --downloader "dash,m3u8:native" will use aria2c for http/ftp downloads, and the native downloader for dash/m3u8 downloads (Alias: --external-downloader)
    pub fn downloader(&mut self, protoname: &str) -> &mut Self {
        self.command_mut().arg("--downloader");
        self.command_mut().arg(protoname);
        self
    }

    /// Give these arguments to the external downloader. Specify the downloader name and the arguments separated by a colon ":". For ffmpeg, arguments can be passed to different positions using the same syntax as --postprocessor-args. You can use this option multiple times to give different arguments to different downloaders (Alias: --external-downloader-args)
    pub fn downloader_args(&mut self, nameargs: &str) -> &mut Self {
        self.command_mut().arg("--downloader-args");
        self.command_mut().arg(nameargs);
        self
    }
}

/// Filesystem Options
impl Builder {
    /// File containing URLs to download ("-" for stdin), one URL per line. Lines starting with "#", ";" or "]" are considered as comments and ignored
    pub fn batch_file(&mut self, file: &str) -> &mut Self {
        self.command_mut().arg("--batch-file");
        self.command_mut().arg(file);
        self
    }

    /// Do not read URLs from batch file (default)
    pub fn no_batch_file(&mut self) -> &mut Self {
        self.command_mut().arg("--no-batch-file");
        self
    }

    /// The paths where the files should be downloaded. Specify the type of file and the path separated by a colon ":". All the same TYPES as --output are supported. Additionally, you can also provide "home" (default) and "temp" paths. All intermediary files are first downloaded to the temp path and then the final files are moved over to the home path after download is finished. This option is ignored if --output is an absolute path
    pub fn paths(&mut self, typespath: &str) -> &mut Self {
        self.command_mut().arg("--paths");
        self.command_mut().arg(typespath);
        self
    }

    /// Output filename template; see "OUTPUT TEMPLATE" for details
    pub fn output(&mut self, typestemplate: &str) -> &mut Self {
        self.command_mut().arg("--output");
        self.command_mut().arg(typestemplate);
        self
    }

    /// Placeholder for unavailable fields in --output (default: "NA")
    pub fn output_na_placeholder(&mut self, text: &str) -> &mut Self {
        self.command_mut().arg("--output-na-placeholder");
        self.command_mut().arg(text);
        self
    }

    /// Restrict filenames to only ASCII characters, and avoid "&" and spaces in filenames
    pub fn restrict_filenames(&mut self) -> &mut Self {
        self.command_mut().arg("--restrict-filenames");
        self
    }

    /// Allow Unicode characters, "&" and spaces in filenames (default)
    pub fn no_restrict_filenames(&mut self) -> &mut Self {
        self.command_mut().arg("--no-restrict-filenames");
        self
    }

    /// Force filenames to be Windows-compatible
    pub fn windows_filenames(&mut self) -> &mut Self {
        self.command_mut().arg("--windows-filenames");
        self
    }

    /// Make filenames Windows-compatible only if using Windows (default)
    pub fn no_windows_filenames(&mut self) -> &mut Self {
        self.command_mut().arg("--no-windows-filenames");
        self
    }

    /// Limit the filename length (excluding extension) to the specified number of characters
    pub fn trim_filenames(&mut self, length: &str) -> &mut Self {
        self.command_mut().arg("--trim-filenames");
        self.command_mut().arg(length);
        self
    }

    /// Do not overwrite any files
    pub fn no_overwrites(&mut self) -> &mut Self {
        self.command_mut().arg("--no-overwrites");
        self
    }

    /// Overwrite all video and metadata files. This option includes --no-continue
    pub fn force_overwrites(&mut self) -> &mut Self {
        self.command_mut().arg("--force-overwrites");
        self
    }

    /// Do not overwrite the video, but overwrite related files (default)
    pub fn no_force_overwrites(&mut self) -> &mut Self {
        self.command_mut().arg("--no-force-overwrites");
        self
    }

    /// Resume partially downloaded files/fragments (default)
    pub fn r#continue(&mut self) -> &mut Self {
        self.command_mut().arg("--continue");
        self
    }

    /// Do not resume partially downloaded fragments. If the file is not fragmented, restart download of the entire file
    pub fn no_continue(&mut self) -> &mut Self {
        self.command_mut().arg("--no-continue");
        self
    }

    /// Use .part files instead of writing directly into output file (default)
    pub fn part(&mut self) -> &mut Self {
        self.command_mut().arg("--part");
        self
    }

    /// Do not use .part files - write directly into output file
    pub fn no_part(&mut self) -> &mut Self {
        self.command_mut().arg("--no-part");
        self
    }

    /// Use the Last-modified header to set the file modification time (default)
    pub fn mtime(&mut self) -> &mut Self {
        self.command_mut().arg("--mtime");
        self
    }

    /// Do not use the Last-modified header to set the file modification time
    pub fn no_mtime(&mut self) -> &mut Self {
        self.command_mut().arg("--no-mtime");
        self
    }

    /// Write video description to a .description file
    pub fn write_description(&mut self) -> &mut Self {
        self.command_mut().arg("--write-description");
        self
    }

    /// Do not write video description (default)
    pub fn no_write_description(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-description");
        self
    }

    /// Write video metadata to a .info.json file (this may contain personal information)
    pub fn write_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--write-info-json");
        self
    }

    /// Do not write video metadata (default)
    pub fn no_write_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-info-json");
        self
    }

    /// Write playlist metadata in addition to the video metadata when using --write-info-json, --write-description etc. (default)
    pub fn write_playlist_metafiles(&mut self) -> &mut Self {
        self.command_mut().arg("--write-playlist-metafiles");
        self
    }

    /// Do not write playlist metadata when using --write-info-json, --write-description etc.
    pub fn no_write_playlist_metafiles(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-playlist-metafiles");
        self
    }

    /// Remove some internal metadata such as filenames from the infojson (default)
    pub fn clean_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--clean-info-json");
        self
    }

    /// Write all fields to the infojson
    pub fn no_clean_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--no-clean-info-json");
        self
    }

    /// Retrieve video comments to be placed in the infojson. The comments are fetched even without this option if the extraction is known to be quick (Alias: --get-comments)
    pub fn write_comments(&mut self) -> &mut Self {
        self.command_mut().arg("--write-comments");
        self
    }

    /// Do not retrieve video comments unless the extraction is known to be quick (Alias: --no-get-comments)
    pub fn no_write_comments(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-comments");
        self
    }

    /// JSON file containing the video information (created with the "--write-info-json" option)
    pub fn load_info_json(&mut self, file: &str) -> &mut Self {
        self.command_mut().arg("--load-info-json");
        self.command_mut().arg(file);
        self
    }

    /// Netscape formatted file to read cookies from and dump cookie jar in
    pub fn cookies(&mut self, file: &str) -> &mut Self {
        self.command_mut().arg("--cookies");
        self.command_mut().arg(file);
        self
    }

    /// Do not read/dump cookies from/to file (default)
    pub fn no_cookies(&mut self) -> &mut Self {
        self.command_mut().arg("--no-cookies");
        self
    }

    ///  The name of the browser to load cookies from. Currently supported browsers are: brave, chrome, chromium, edge, firefox, opera, safari, vivaldi, whale. Optionally, the KEYRING used for decrypting Chromium cookies on Linux, the name/path of the PROFILE to load cookies from, and the CONTAINER name (if Firefox) ("none" for no container) can be given with their respective separators. By default, all containers of the most recently accessed profile are used. Currently supported keyrings are: basictext, gnomekeyring, kwallet, kwallet5, kwallet6
    pub fn cookies_from_browser(&mut self, browserkeyringprofilecontainer: &str) -> &mut Self {
        self.command_mut().arg("--cookies-from-browser");
        self.command_mut().arg(browserkeyringprofilecontainer);
        self
    }

    /// Do not load cookies from browser (default)
    pub fn no_cookies_from_browser(&mut self) -> &mut Self {
        self.command_mut().arg("--no-cookies-from-browser");
        self
    }

    /// Location in the filesystem where yt-dlp can store some downloaded information (such as client ids and signatures) permanently. By default ${XDG_CACHE_HOME}/yt-dlp
    pub fn cache_dir(&mut self, dir: &str) -> &mut Self {
        self.command_mut().arg("--cache-dir");
        self.command_mut().arg(dir);
        self
    }

    /// Disable filesystem caching
    pub fn no_cache_dir(&mut self) -> &mut Self {
        self.command_mut().arg("--no-cache-dir");
        self
    }

    /// Delete all filesystem cache files
    pub fn rm_cache_dir(&mut self) -> &mut Self {
        self.command_mut().arg("--rm-cache-dir");
        self
    }
}

/// Thumbnail Options
impl Builder {
    /// Write thumbnail image to disk
    pub fn write_thumbnail(&mut self) -> &mut Self {
        self.command_mut().arg("--write-thumbnail");
        self
    }

    /// Do not write thumbnail image to disk (default)
    pub fn no_write_thumbnail(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-thumbnail");
        self
    }

    /// Write all thumbnail image formats to disk
    pub fn write_all_thumbnails(&mut self) -> &mut Self {
        self.command_mut().arg("--write-all-thumbnails");
        self
    }

    /// List available thumbnails of each video. Simulate unless --no-simulate is used
    pub fn list_thumbnails(&mut self) -> &mut Self {
        self.command_mut().arg("--list-thumbnails");
        self
    }
}

/// Internet Shortcut Options
impl Builder {
    /// Write an internet shortcut file, depending on the current platform (.url, .webloc or .desktop). The URL may be cached by the OS
    pub fn write_link(&mut self) -> &mut Self {
        self.command_mut().arg("--write-link");
        self
    }

    /// Write a .url Windows internet shortcut. The OS caches the URL based on the file path
    pub fn write_url_link(&mut self) -> &mut Self {
        self.command_mut().arg("--write-url-link");
        self
    }

    /// Write a .webloc macOS internet shortcut
    pub fn write_webloc_link(&mut self) -> &mut Self {
        self.command_mut().arg("--write-webloc-link");
        self
    }

    /// Write a .desktop Linux internet shortcut
    pub fn write_desktop_link(&mut self) -> &mut Self {
        self.command_mut().arg("--write-desktop-link");
        self
    }
}

/// Verbosity and Simulation Options
impl Builder {
    /// Activate quiet mode. If used with --verbose, print the log to stderr
    pub fn quiet(&mut self) -> &mut Self {
        self.command_mut().arg("--quiet");
        self
    }

    /// Deactivate quiet mode. (Default)
    pub fn no_quiet(&mut self) -> &mut Self {
        self.command_mut().arg("--no-quiet");
        self
    }

    /// Ignore warnings
    pub fn no_warnings(&mut self) -> &mut Self {
        self.command_mut().arg("--no-warnings");
        self
    }

    /// Do not download the video and do not write anything to disk
    pub fn simulate(&mut self) -> &mut Self {
        self.command_mut().arg("--simulate");
        self
    }

    /// Download the video even if printing/listing options are used
    pub fn no_simulate(&mut self) -> &mut Self {
        self.command_mut().arg("--no-simulate");
        self
    }

    /// Ignore "No video formats" error. Useful for extracting metadata even if the videos are not actually available for download (experimental)
    pub fn ignore_no_formats_error(&mut self) -> &mut Self {
        self.command_mut().arg("--ignore-no-formats-error");
        self
    }

    /// Throw error when no downloadable video formats are found (default)
    pub fn no_ignore_no_formats_error(&mut self) -> &mut Self {
        self.command_mut().arg("--no-ignore-no-formats-error");
        self
    }

    /// Do not download the video but write all related files (Alias: --no-download)
    pub fn skip_download(&mut self) -> &mut Self {
        self.command_mut().arg("--skip-download");
        self
    }

    /// Field name or output template to print to screen, optionally prefixed with when to print it, separated by a ":". Supported values of "WHEN" are the same as that of --use-postprocessor (default: video). Implies --quiet. Implies --simulate unless --no-simulate or later stages of WHEN are used. This option can be used multiple times
    pub fn print(&mut self, whentemplate: &str) -> &mut Self {
        self.command_mut().arg("--print");
        self.command_mut().arg(whentemplate);
        self
    }

    /// FILE Append given template to the file. The values of WHEN and TEMPLATE are same as that of --print. FILE uses the same syntax as the output template. This option can be used multiple times
    pub fn print_to_file(&mut self, whentemplate: &str) -> &mut Self {
        self.command_mut().arg("--print-to-file");
        self.command_mut().arg(whentemplate);
        self
    }

    /// Quiet, but print JSON information for each video. Simulate unless --no-simulate is used. See "OUTPUT TEMPLATE" for a description of available keys
    pub fn dump_json(&mut self) -> &mut Self {
        self.command_mut().arg("--dump-json");
        self
    }

    /// Quiet, but print JSON information for each url or infojson passed. Simulate unless --no-simulate is used. If the URL refers to a playlist, the whole playlist information is dumped in a single line
    pub fn dump_single_json(&mut self) -> &mut Self {
        self.command_mut().arg("--dump-single-json");
        self
    }

    /// Force download archive entries to be written as far as no errors occur, even if -s or another simulation option is used (Alias: --force-download-archive)
    pub fn force_write_archive(&mut self) -> &mut Self {
        self.command_mut().arg("--force-write-archive");
        self
    }

    /// Output progress bar as new lines
    pub fn newline(&mut self) -> &mut Self {
        self.command_mut().arg("--newline");
        self
    }

    /// Do not print progress bar
    pub fn no_progress(&mut self) -> &mut Self {
        self.command_mut().arg("--no-progress");
        self
    }

    /// Show progress bar, even if in quiet mode
    pub fn progress(&mut self) -> &mut Self {
        self.command_mut().arg("--progress");
        self
    }

    /// Display progress in console titlebar
    pub fn console_title(&mut self) -> &mut Self {
        self.command_mut().arg("--console-title");
        self
    }

    ///  Template for progress outputs, optionally prefixed with one of "download:" (default), "download-title:" (the console title), "postprocess:",  or "postprocess-title:". The video's fields are accessible under the "info" key and the progress attributes are accessible under "progress" key. E.g. --console-title --progress-template "download- title:%(info.id)s-%(progress.eta)s"
    pub fn progress_template(&mut self, typestemplate: &str) -> &mut Self {
        self.command_mut().arg("--progress-template");
        self.command_mut().arg(typestemplate);
        self
    }

    /// Time between progress output (default: 0)
    pub fn progress_delta(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--progress-delta");
        self.command_mut().arg(seconds);
        self
    }

    /// Print various debugging information
    pub fn verbose(&mut self) -> &mut Self {
        self.command_mut().arg("--verbose");
        self
    }

    /// Print downloaded pages encoded using base64 to debug problems (very verbose)
    pub fn dump_pages(&mut self) -> &mut Self {
        self.command_mut().arg("--dump-pages");
        self
    }

    /// Write downloaded intermediary pages to files in the current directory to debug problems
    pub fn write_pages(&mut self) -> &mut Self {
        self.command_mut().arg("--write-pages");
        self
    }

    /// Display sent and read HTTP traffic
    pub fn print_traffic(&mut self) -> &mut Self {
        self.command_mut().arg("--print-traffic");
        self
    }

    /// Force the specified encoding (experimental)
    pub fn encoding(&mut self, encoding: &str) -> &mut Self {
        self.command_mut().arg("--encoding");
        self.command_mut().arg(encoding);
        self
    }

    /// Explicitly allow HTTPS connection to servers that do not support RFC 5746 secure renegotiation
    pub fn legacy_server_connect(&mut self) -> &mut Self {
        self.command_mut().arg("--legacy-server-connect");
        self
    }

    /// Suppress HTTPS certificate validation
    pub fn no_check_certificates(&mut self) -> &mut Self {
        self.command_mut().arg("--no-check-certificates");
        self
    }

    /// Use an unencrypted connection to retrieve information about the video (Currently supported only for YouTube)
    pub fn prefer_insecure(&mut self) -> &mut Self {
        self.command_mut().arg("--prefer-insecure");
        self
    }

    /// Specify a custom HTTP header and its value, separated by a colon ":". You can use this option multiple times
    pub fn add_headers(&mut self, fieldvalue: &str) -> &mut Self {
        self.command_mut().arg("--add-headers");
        self.command_mut().arg(fieldvalue);
        self
    }

    /// Work around terminals that lack bidirectional text support. Requires bidiv or fribidi executable in PATH
    pub fn bidi_workaround(&mut self) -> &mut Self {
        self.command_mut().arg("--bidi-workaround");
        self
    }

    /// Number of seconds to sleep between requests during data extraction
    pub fn sleep_requests(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--sleep-requests");
        self.command_mut().arg(seconds);
        self
    }

    /// Number of seconds to sleep before each download. This is the minimum time to sleep when used along with --max-sleep-interval (Alias: --min-sleep-interval)
    pub fn sleep_interval(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--sleep-interval");
        self.command_mut().arg(seconds);
        self
    }

    /// Maximum number of seconds to sleep. Can only be used along with --min-sleep-interval
    pub fn max_sleep_interval(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--max-sleep-interval");
        self.command_mut().arg(seconds);
        self
    }

    /// Number of seconds to sleep before each subtitle download
    pub fn sleep_subtitles(&mut self, seconds: &str) -> &mut Self {
        self.command_mut().arg("--sleep-subtitles");
        self.command_mut().arg(seconds);
        self
    }
}

/// Video Format Options
impl Builder {
    /// Video format code, see "FORMAT SELECTION" for more details
    pub fn format(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--format");
        self.command_mut().arg(format);
        self
    }

    /// Sort the formats by the fields given, see "Sorting Formats" for more details
    pub fn format_sort(&mut self, sortorder: &str) -> &mut Self {
        self.command_mut().arg("--format-sort");
        self.command_mut().arg(sortorder);
        self
    }

    /// Force user specified sort order to have precedence over all fields, see "Sorting Formats" for more details (Alias: --S-force)
    pub fn format_sort_force(&mut self) -> &mut Self {
        self.command_mut().arg("--format-sort-force");
        self
    }

    /// Some fields have precedence over the user specified sort order (default)
    pub fn no_format_sort_force(&mut self) -> &mut Self {
        self.command_mut().arg("--no-format-sort-force");
        self
    }

    /// Allow multiple video streams to be merged into a single file
    pub fn video_multistreams(&mut self) -> &mut Self {
        self.command_mut().arg("--video-multistreams");
        self
    }

    /// Only one video stream is downloaded for each output file (default)
    pub fn no_video_multistreams(&mut self) -> &mut Self {
        self.command_mut().arg("--no-video-multistreams");
        self
    }

    /// Allow multiple audio streams to be merged into a single file
    pub fn audio_multistreams(&mut self) -> &mut Self {
        self.command_mut().arg("--audio-multistreams");
        self
    }

    /// Only one audio stream is downloaded for each output file (default)
    pub fn no_audio_multistreams(&mut self) -> &mut Self {
        self.command_mut().arg("--no-audio-multistreams");
        self
    }

    /// Prefer video formats with free containers over non-free ones of same quality. Use with "-S ext" to strictly prefer free containers irrespective of quality
    pub fn prefer_free_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--prefer-free-formats");
        self
    }

    /// Don't give any special preference to free containers (default)
    pub fn no_prefer_free_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--no-prefer-free-formats");
        self
    }

    /// Make sure formats are selected only from those that are actually downloadable
    pub fn check_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--check-formats");
        self
    }

    /// Check all formats for whether they are actually downloadable
    pub fn check_all_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--check-all-formats");
        self
    }

    /// Do not check that the formats are actually downloadable
    pub fn no_check_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--no-check-formats");
        self
    }

    /// List available formats of each video. Simulate unless --no-simulate is used
    pub fn list_formats(&mut self) -> &mut Self {
        self.command_mut().arg("--list-formats");
        self
    }

    /// Containers that may be used when merging formats, separated by "/", e.g. "mp4/mkv". Ignored if no merge is required. (currently supported: avi, flv, mkv, mov, mp4, webm)
    pub fn merge_output_format(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--merge-output-format");
        self.command_mut().arg(format);
        self
    }
}

/// Subtitle Options
impl Builder {
    /// Write subtitle file
    pub fn write_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--write-subs");
        self
    }

    /// Do not write subtitle file (default)
    pub fn no_write_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-subs");
        self
    }

    /// Write automatically generated subtitle file (Alias: --write-automatic-subs)
    pub fn write_auto_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--write-auto-subs");
        self
    }

    /// Do not write auto-generated subtitles (default) (Alias: --no-write-automatic-subs)
    pub fn no_write_auto_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--no-write-auto-subs");
        self
    }

    /// List available subtitles of each video. Simulate unless --no-simulate is used
    pub fn list_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--list-subs");
        self
    }

    /// Subtitle format; accepts formats preference, e.g. "srt" or "ass/srt/best"
    pub fn sub_format(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--sub-format");
        self.command_mut().arg(format);
        self
    }

    /// Languages of the subtitles to download (can be regex) or "all" separated by commas, e.g. --sub-langs "en.*,ja". You can prefix the language code with a "-" to exclude it from the requested languages, e.g. --sub-langs all,-live_chat. Use --list-subs for a list of available language tags
    pub fn sub_langs(&mut self, langs: &str) -> &mut Self {
        self.command_mut().arg("--sub-langs");
        self.command_mut().arg(langs);
        self
    }
}

/// Authentication Options
impl Builder {
    /// Login with this account ID
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.command_mut().arg("--username");
        self.command_mut().arg(username);
        self
    }

    /// Account password. If this option is left out, yt-dlp will ask interactively
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.command_mut().arg("--password");
        self.command_mut().arg(password);
        self
    }

    /// Two-factor authentication code
    pub fn twofactor(&mut self, twofactor: &str) -> &mut Self {
        self.command_mut().arg("--twofactor");
        self.command_mut().arg(twofactor);
        self
    }

    /// Use .netrc authentication data
    pub fn netrc(&mut self) -> &mut Self {
        self.command_mut().arg("--netrc");
        self
    }

    /// Location of .netrc authentication data; either the path or its containing directory. Defaults to ~/.netrc
    pub fn netrc_location(&mut self, path: &str) -> &mut Self {
        self.command_mut().arg("--netrc-location");
        self.command_mut().arg(path);
        self
    }

    /// Command to execute to get the credentials for an extractor.
    pub fn netrc_cmd(&mut self, netrccmd: &str) -> &mut Self {
        self.command_mut().arg("--netrc-cmd");
        self.command_mut().arg(netrccmd);
        self
    }

    /// Video-specific password
    pub fn video_password(&mut self, password: &str) -> &mut Self {
        self.command_mut().arg("--video-password");
        self.command_mut().arg(password);
        self
    }

    /// Adobe Pass multiple-system operator (TV provider) identifier, use --ap-list-mso for a list of available MSOs
    pub fn ap_mso(&mut self, mso: &str) -> &mut Self {
        self.command_mut().arg("--ap-mso");
        self.command_mut().arg(mso);
        self
    }

    /// Multiple-system operator account login
    pub fn ap_username(&mut self, username: &str) -> &mut Self {
        self.command_mut().arg("--ap-username");
        self.command_mut().arg(username);
        self
    }

    /// Multiple-system operator account password. If this option is left out, yt-dlp will ask interactively
    pub fn ap_password(&mut self, password: &str) -> &mut Self {
        self.command_mut().arg("--ap-password");
        self.command_mut().arg(password);
        self
    }

    /// List all supported multiple-system operators
    pub fn ap_list_mso(&mut self) -> &mut Self {
        self.command_mut().arg("--ap-list-mso");
        self
    }

    /// Path to client certificate file in PEM format. May include the private key
    pub fn client_certificate(&mut self, certfile: &str) -> &mut Self {
        self.command_mut().arg("--client-certificate");
        self.command_mut().arg(certfile);
        self
    }

    ///  Path to private key file for client certificate
    pub fn client_certificate_key(&mut self, keyfile: &str) -> &mut Self {
        self.command_mut().arg("--client-certificate-key");
        self.command_mut().arg(keyfile);
        self
    }

    ///  Password for client certificate private key, if encrypted. If not provided, and the key is encrypted, yt-dlp will ask interactively
    pub fn client_certificate_password(&mut self, password: &str) -> &mut Self {
        self.command_mut().arg("--client-certificate-password");
        self.command_mut().arg(password);
        self
    }
}

/// Post-Processing Options
impl Builder {
    /// Convert video files to audio-only files (requires ffmpeg and ffprobe)
    pub fn extract_audio(&mut self) -> &mut Self {
        self.command_mut().arg("--extract-audio");
        self
    }

    /// Format to convert the audio to when -x is used. (currently supported: best (default), aac, alac, flac, m4a, mp3, opus, vorbis, wav). You can specify multiple rules using similar syntax as --remux-video
    pub fn audio_format(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--audio-format");
        self.command_mut().arg(format);
        self
    }

    /// Specify ffmpeg audio quality to use when converting the audio with -x. Insert a value between 0 (best) and 10 (worst) for VBR or a specific bitrate like 128K (default 5)
    pub fn audio_quality(&mut self, quality: &str) -> &mut Self {
        self.command_mut().arg("--audio-quality");
        self.command_mut().arg(quality);
        self
    }

    /// Remux the video into another container if necessary (currently supported: avi, flv, gif, mkv, mov, mp4, webm, aac, aiff, alac, flac, m4a, mka, mp3, ogg, opus, vorbis, wav). If target container does not support the video/audio codec, remuxing will fail. You can specify multiple rules; e.g. "aac>m4a/mov>mp4/mkv" will remux aac to m4a, mov to mp4 and anything else to mkv
    pub fn remux_video(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--remux-video");
        self.command_mut().arg(format);
        self
    }

    /// Re-encode the video into another format if necessary. The syntax and supported formats are the same as --remux-video
    pub fn recode_video(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--recode-video");
        self.command_mut().arg(format);
        self
    }

    /// Give these arguments to the postprocessors. Specify the postprocessor/executable name and the arguments separated by a colon ":" to give the argument to the specified postprocessor/executable. Supported PP are: Merger, ModifyChapters, SplitChapters, ExtractAudio, VideoRemuxer, VideoConvertor, Metadata, EmbedSubtitle, EmbedThumbnail, SubtitlesConvertor, ThumbnailsConvertor, FixupStretched, FixupM4a, FixupM3u8, FixupTimestamp and FixupDuration. The supported executables are: AtomicParsley, FFmpeg and FFprobe. You can also specify "PP+EXE:ARGS" to give the arguments to the specified executable only when being used by the specified postprocessor. Additionally, for ffmpeg/ffprobe, "_i"/"_o" can be appended to the prefix optionally followed by a number to pass the argument before the specified input/output file, e.g. --ppa "Merger+ffmpeg_i1:-v quiet". You can use this option multiple times to give different arguments to different postprocessors. (Alias: --ppa)
    pub fn postprocessor_args(&mut self, nameargs: &str) -> &mut Self {
        self.command_mut().arg("--postprocessor-args");
        self.command_mut().arg(nameargs);
        self
    }

    /// Keep the intermediate video file on disk after post-processing
    pub fn keep_video(&mut self) -> &mut Self {
        self.command_mut().arg("--keep-video");
        self
    }

    /// Delete the intermediate video file after post-processing (default)
    pub fn no_keep_video(&mut self) -> &mut Self {
        self.command_mut().arg("--no-keep-video");
        self
    }

    /// Overwrite post-processed files (default)
    pub fn post_overwrites(&mut self) -> &mut Self {
        self.command_mut().arg("--post-overwrites");
        self
    }

    /// Do not overwrite post-processed files
    pub fn no_post_overwrites(&mut self) -> &mut Self {
        self.command_mut().arg("--no-post-overwrites");
        self
    }

    /// Embed subtitles in the video (only for mp4, webm and mkv videos)
    pub fn embed_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--embed-subs");
        self
    }

    /// Do not embed subtitles (default)
    pub fn no_embed_subs(&mut self) -> &mut Self {
        self.command_mut().arg("--no-embed-subs");
        self
    }

    /// Embed thumbnail in the video as cover art
    pub fn embed_thumbnail(&mut self) -> &mut Self {
        self.command_mut().arg("--embed-thumbnail");
        self
    }

    /// Do not embed thumbnail (default)
    pub fn no_embed_thumbnail(&mut self) -> &mut Self {
        self.command_mut().arg("--no-embed-thumbnail");
        self
    }

    /// Embed metadata to the video file. Also embeds chapters/infojson if present unless --no-embed-chapters/--no-embed-info-json are used (Alias: --add-metadata)
    pub fn embed_metadata(&mut self) -> &mut Self {
        self.command_mut().arg("--embed-metadata");
        self
    }

    /// Do not add metadata to file (default) (Alias: --no-add-metadata)
    pub fn no_embed_metadata(&mut self) -> &mut Self {
        self.command_mut().arg("--no-embed-metadata");
        self
    }

    /// Add chapter markers to the video file (Alias: --add-chapters)
    pub fn embed_chapters(&mut self) -> &mut Self {
        self.command_mut().arg("--embed-chapters");
        self
    }

    /// Do not add chapter markers (default) (Alias: --no-add-chapters)
    pub fn no_embed_chapters(&mut self) -> &mut Self {
        self.command_mut().arg("--no-embed-chapters");
        self
    }

    /// Embed the infojson as an attachment to mkv/mka video files
    pub fn embed_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--embed-info-json");
        self
    }

    /// Do not embed the infojson as an attachment to the video file
    pub fn no_embed_info_json(&mut self) -> &mut Self {
        self.command_mut().arg("--no-embed-info-json");
        self
    }

    ///  Parse additional metadata like title/artist from other fields; see "MODIFYING METADATA" for details. Supported values of "WHEN" are the same as that of --use-postprocessor (default: pre_process)
    pub fn parse_metadata(&mut self, whenfromto: &str) -> &mut Self {
        self.command_mut().arg("--parse-metadata");
        self.command_mut().arg(whenfromto);
        self
    }

    ///  Replace text in a metadata field using the given regex. This option can be used multiple times. Supported values of "WHEN" are the same as that of --use-postprocessor (default: pre_process)
    pub fn replace_in_metadata(&mut self, replace: &str) -> &mut Self {
        self.command_mut().arg("--replace-in-metadata");
        self.command_mut().arg(replace);
        self
    }

    /// Write metadata to the video file's xattrs (using dublin core and xdg standards)
    pub fn xattrs(&mut self) -> &mut Self {
        self.command_mut().arg("--xattrs");
        self
    }

    /// Concatenate videos in a playlist. One of "never", "always", or "multi_video" (default; only when the videos form a single show). All the video files must have same codecs and number of streams to be concatable. The "pl_video:" prefix can be used with "--paths" and "--output" to set the output filename for the concatenated files. See "OUTPUT TEMPLATE" for details
    pub fn concat_playlist(&mut self, policy: &str) -> &mut Self {
        self.command_mut().arg("--concat-playlist");
        self.command_mut().arg(policy);
        self
    }

    /// Automatically correct known faults of the file. One of never (do nothing), warn (only emit a warning), detect_or_warn (the default; fix file if we can, warn otherwise), force (try fixing even if file already exists)
    pub fn fixup(&mut self, policy: &str) -> &mut Self {
        self.command_mut().arg("--fixup");
        self.command_mut().arg(policy);
        self
    }

    /// Location of the ffmpeg binary; either the path to the binary or its containing directory
    pub fn ffmpeg_location(&mut self, path: &str) -> &mut Self {
        self.command_mut().arg("--ffmpeg-location");
        self.command_mut().arg(path);
        self
    }

    /// Execute a command, optionally prefixed with when to execute it, separated by a ":". Supported values of "WHEN" are the same as that of --use-postprocessor (default: after_move). Same syntax as the output template can be used to pass any field as arguments to the command. If no fields are passed, %(filepath,_filename|)q is appended to the end of the command. This option can be used multiple times
    pub fn exec(&mut self, whencmd: &str) -> &mut Self {
        self.command_mut().arg("--exec");
        self.command_mut().arg(whencmd);
        self
    }

    /// Remove any previously defined --exec
    pub fn no_exec(&mut self) -> &mut Self {
        self.command_mut().arg("--no-exec");
        self
    }

    /// Convert the subtitles to another format (currently supported: ass, lrc, srt, vtt) (Alias: --convert-subtitles)
    pub fn convert_subs(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--convert-subs");
        self.command_mut().arg(format);
        self
    }

    /// Convert the thumbnails to another format (currently supported: jpg, png, webp). You can specify multiple rules using similar syntax as --remux-video
    pub fn convert_thumbnails(&mut self, format: &str) -> &mut Self {
        self.command_mut().arg("--convert-thumbnails");
        self.command_mut().arg(format);
        self
    }

    /// Split video into multiple files based on internal chapters. The "chapter:" prefix can be used with "--paths" and "--output" to set the output filename for the split files. See "OUTPUT TEMPLATE" for details
    pub fn split_chapters(&mut self) -> &mut Self {
        self.command_mut().arg("--split-chapters");
        self
    }

    /// Do not split video based on chapters (default)
    pub fn no_split_chapters(&mut self) -> &mut Self {
        self.command_mut().arg("--no-split-chapters");
        self
    }

    /// Remove chapters whose title matches the given regular expression. The syntax is the same as --download-sections. This option can be used multiple times
    pub fn remove_chapters(&mut self, regex: &str) -> &mut Self {
        self.command_mut().arg("--remove-chapters");
        self.command_mut().arg(regex);
        self
    }

    /// Do not remove any chapters from the file (default)
    pub fn no_remove_chapters(&mut self) -> &mut Self {
        self.command_mut().arg("--no-remove-chapters");
        self
    }

    /// Force keyframes at cuts when downloading/splitting/removing sections. This is slow due to needing a re-encode, but the resulting video may have fewer artifacts around the cuts
    pub fn force_keyframes_at_cuts(&mut self) -> &mut Self {
        self.command_mut().arg("--force-keyframes-at-cuts");
        self
    }

    /// Do not force keyframes around the chapters when cutting/splitting (default)
    pub fn no_force_keyframes_at_cuts(&mut self) -> &mut Self {
        self.command_mut().arg("--no-force-keyframes-at-cuts");
        self
    }

    ///  The (case sensitive) name of plugin postprocessors to be enabled, and (optionally) arguments to be passed to it, separated by a colon ":". ARGS are a semicolon ";" delimited list of NAME=VALUE. The "when" argument determines when the postprocessor is invoked. It can be one of "pre_process" (after video extraction), "after_filter" (after video passes filter), "video" (after --format; before --print/--output), "before_dl" (before each video download), "post_process" (after each video download; default), "after_move" (after moving video file to its final locations), "after_video" (after downloading and processing all formats of a video), or "playlist" (at end of playlist). This option can be used multiple times to add different postprocessors
    pub fn use_postprocessor(&mut self, nameargs: &str) -> &mut Self {
        self.command_mut().arg("--use-postprocessor");
        self.command_mut().arg(nameargs);
        self
    }
}

/// SponsorBlock Options   
impl Builder {
    /// SponsorBlock categories to create chapters for, separated by commas. Available categories are sponsor, intro, outro, selfpromo, preview, filler, interaction, music_offtopic, poi_highlight, chapter, all and default (=all). You can prefix the category with a "-" to exclude it. See [1] for description of the categories. E.g. --sponsorblock-mark all,-preview [1] https:/ /wiki.sponsor.ajay.app/w/Segment_Categories
    pub fn sponsorblock_mark(&mut self, cats: &str) -> &mut Self {
        self.command_mut().arg("--sponsorblock-mark");
        self.command_mut().arg(cats);
        self
    }

    /// SponsorBlock categories to be removed from the video file, separated by commas. If a category is present in both mark and remove, remove takes precedence. The syntax and available categories are the same as for --sponsorblock-mark except that "default" refers to "all,-filler" and poi_highlight, chapter are not available
    pub fn sponsorblock_remove(&mut self, cats: &str) -> &mut Self {
        self.command_mut().arg("--sponsorblock-remove");
        self.command_mut().arg(cats);
        self
    }

    ///  An output template for the title of the SponsorBlock chapters created by --sponsorblock-mark. The only available fields are start_time, end_time, category, categories, name, category_names. Defaults to "[SponsorBlock]: %(category_names)l"
    pub fn sponsorblock_chapter_title(&mut self, template: &str) -> &mut Self {
        self.command_mut().arg("--sponsorblock-chapter-title");
        self.command_mut().arg(template);
        self
    }

    /// Disable both --sponsorblock-mark and --sponsorblock-remove
    pub fn no_sponsorblock(&mut self) -> &mut Self {
        self.command_mut().arg("--no-sponsorblock");
        self
    }

    /// SponsorBlock API location, defaults to https://sponsor.ajay.app
    pub fn sponsorblock_api(&mut self, url: &str) -> &mut Self {
        self.command_mut().arg("--sponsorblock-api");
        self.command_mut().arg(url);
        self
    }
}
