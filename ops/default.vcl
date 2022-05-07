vcl 4.1;

import std;
import bodyaccess;
import stale;

backend hifumin {
    .host = "127.0.0.1";
    .port = "8080";
    .max_connections = 1500;
    .probe = {
        .url = "/";
        .interval = 1s;
        .timeout = 3s;
    }
    .connect_timeout        = 30s;
    .first_byte_timeout     = 30s;
    .between_bytes_timeout  = 3s;
}


sub vcl_recv {
    unset req.http.X-Body-Len;

    if (req.method == "POST") {
        std.cache_req_body(2KB);
        set req.http.X-Body-Len = bodyaccess.len_req_body();
        if (req.http.X-Body-Len == "-1") {
            return(synth(400, "The request body size exceeds the limit"));
        }

        return (hash);
    }
}

sub vcl_hash {
    # To cache POST and PUT requests
    if (req.http.X-Body-Len) {
        bodyaccess.hash_req_body();
    } else {
        hash_data("");
    }
}

sub vcl_backend_fetch {
    if (bereq.http.X-Body-Len) {
        set bereq.method = "POST";
    }
}

sub vcl_backend_response {
    set beresp.ttl = 1d;

    set beresp.grace = 30s;
    set beresp.keep = 30s;

    set beresp.stale_while_revalidate = 30s;
    set beresp.stale_if_error = 30s;
}

sub stale_if_error {
    if (beresp.status >= 500 && stale.exists()) {
        # Tune this value to match your traffic and caching patterns
        stale.revive(1s, 10m);
        stale.deliver();
        return (abandon);
    }
}

sub vcl_backend_response {
    call stale_if_error;
}

sub vcl_backend_error {
    call stale_if_error;
}