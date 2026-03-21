(function () {
  const nativeFetch = window.fetch;

  window.fetch = function (input, init) {
    const url = typeof input === "string" ? input : input.url;

    // Only intercept /run requests
    if (!url.startsWith("/run")) {
      return nativeFetch.call(window, input, init);
    }

    const body = init && init.body ? init.body : "{}";
    const encoder = new TextEncoder();

    const stream = new ReadableStream({
      start(controller) {
        const channel = new window.__TAURI__.core.Channel();

        channel.onmessage = function (data) {
          try {
            const parsed = JSON.parse(data);
            if (parsed.event === "end") {
              controller.close();
              return;
            }
          } catch {}
          // Format as SSE so existing parsing code works
          controller.enqueue(encoder.encode("data:" + data + "\n\n"));
        };

        window.__TAURI__.core
          .invoke("stream_run", { body: body, onEvent: channel })
          .catch(function (err) {
            controller.error(err);
          });
      },
    });

    return Promise.resolve(
      new Response(stream, {
        status: 200,
        headers: { "Content-Type": "text/event-stream" },
      })
    );
  };
})();
