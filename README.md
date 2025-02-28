# Stackoverflow Stats Badge For Your GitHub Profile 🚀

![stack overflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year&mini=true)

![stack overflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year&mini=false)

Example badge in use on my [GitHub profile](https://github.com/mattrltrent).

## Usage

1. Copy/paste into your Markdown: `![stack overflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=YOUR_USERNAME&period=PERIOD&mini=IS_MINI)`.
2. Change `YOUR_USERNAME` to be your account ID. This is found by going to your Stack Overflow profile and getting the number in the URL. For example, for my account I'd go [here](https://stackoverflow.com/users/13029516/matthew-trent), and then see `13029516` in the URL.
3. Change `PERIOD` to be one of: `day`,  `week`, `month`, `quarter`, or `year`. This is how we calculate the "+XXX this day/week/month/quarter/year" on the big badge.
4. Change `IS_MINI` to be either `true` or `false`. A mini badge ignores the `PERIOD` value, but still requires it. 

## Limitations

These restrictions shouldn't interfere with regular usage:

- The badge is cached for 15 minutes, so its data can possibly lag 15 minutes behind your true Stackoverflow stats. This is done to reduce server load.
- You're rate limited to loading 15 different stat badges every 60 seconds based on your IP.
- The StackAPI limits everyone to loading 300 different profiles every 24 hours based on IP.

## Feel free to create issues or PRs!

- Issues [here](https://github.com/mattrltrent/stackoverflow_badge/issues).
- PRs [here](https://github.com/mattrltrent/stackoverflow_badge/pulls).
- Thanks [@yubinjodev](https://github.com/yubinjodev) for the mini badge suggestion.
