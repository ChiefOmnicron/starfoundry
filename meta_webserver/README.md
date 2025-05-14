# Meta-Webserver

When inserting a link into tools like Discord, Slack, and other tools, they will try to create a preview.
To generate the preview it will use the Meta attributes and the Title attribute.
The meta-webserver is a minimal application which only job it is to generate and return an index.html file which includes the necessary meta and title tags for bots to do their previews.

## Problem

There are several problems that prevent the already existing webapp to handle these kind of bot requests:
- The appraisals are dynamically generated, and the actual content, will only be known after the page is loaded
- The application is written in Vue, which requires the JavaScript to be loaded
- Bots will usually only load the HTML file, and nothing else besides that, so the Vue part of the webapp is never loaded by the bots
- In order to set the meta-tags dynamically, Vue needs to be loaded

The core problem is that the JavaScript files are not loaded, which prevents the dynamic setting of meta-tags.

## Solution

The first logical solution is either the use of an external server that holds a certain state of the application that includes the meta-tags.
This would add unnecessary financial costs, and would add some additional problems.

Another solution is to switch to Server Side Rendering (SSR).
For unknown reasons, I was against using this method.
The webapp has grown a lot and I feared that it would require lots of rewriting.

The third solution, is to redirect the bot users to a separate server, which can generate an HTML file with the included meta-tags, something similar to what external service could do.
Bot users can be filtered by their User-Agent in Nginx or any other reverse proxy.
If a human user clicks on the link, they will be redirected to the actual webapp, if an bot requests the link, it will instead be redirected to the webserver for generating the HTML file.

## Nginx changes

As mentioned, the Nginx configuration needs to be adjusted to redirect bot users.
A very short example is shown below.

``` nginx
# will check the user agent against a list of possible bot names
map $http_user_agent $is_user_bot {
  default 0;
  # it's a regex that just tries to detect a bot, some will probably slip through and not be redirected, but that fine
  ~*(bot|crawl|spider|scrape|80legs|archiver|voyager|curl|wget|yahoo|slurp|google|facebook|linkedin|twitter|bing|yandex|whatsapp|share|rss|validator|checker|proxy|seo|webmon|preview|search) 1;
}

server {
  location / {
    # if a bot is detected, it will reroute to localhost:5001, otherwise to localhost:8080
    # you can test it with curl or wget
    if ($is_user_bot) {
        proxy_pass http://localhost:5001;
    }

    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_pass http://localhost:8080;
  }
}
```

## Example return

The server will return something like:

``` html
<!DOCTYPE html>
    <html>
        <head>
            <title>Appraisal OtHjLera7v: Buy 3.60 Million / Sell 3.69 Million</title>

            <meta name="description" content="Entropic Radiation Sink II - 1">
    </head>
</html>
```

Because only bot servers are seeing this page, it can be as minimal as possible.

## Supported routes

- `/appraisals/{appraisal_uuid}`
