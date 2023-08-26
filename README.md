# Stack Overflow Stats Badge For Your GitHub Profile 🚀

## A GitHub ⭐️ is appreciated if this helps you!

![stack overflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year)

Example of it on my GitHub profile [here](https://github.com/mattrltrent).

## Simple to add:

1. Go to [Stack Overflow](https://stackoverflow.com/), then click on your profile. Once on your profile, you should see a number in the URL bar. This is your **account ID**.

   For example, here is my account ID:

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_2.jpeg?raw=true"  style="display: inline"/>

2. Displayed on the stats badge is "*+ some number of reputation per some time period*". You need to choose over what time period you want us to calculate your reputation gain. The options are `day`,  `week`, `month`, `quarter`, and `year`. This is your **period**.

3. Take the base URL, and insert your newly found **account ID** and **period** where appropriate: 

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_3.jpg?raw=true"  style="display: inline"/>

   For example, here is the URL for my account if I wanted to show my reputation gain quarterly:

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_4.jpg?raw=true"  style="display: inline"/>

4. Insert this text into your GitHub profile's `README.md`, obviously with your own failure text and personalized URL:

   `![some text to render if the image fails to load](YOUR URL)`

5. If you're still struggling, check out how I added it to my own profile's `README.md` [here](https://github.com/mattrltrent/mattrltrent/blob/main/README.md).

## Limitations:

These restrictions shouldn't interfere with regular usage:

- The badge is cached for 15 minutes, so its data can possibly lag 15 minutes behind your true Stack Overflow stats. This is done to reduce server load.
- You're rate limited to loading 15 different stat badges every 60 seconds based on your IP.
- The StackAPI limits everyone to loading 300 different profiles every 24 hours based on IP.

## Feel free to create issues or PRs!

- Issues [here](https://github.com/mattrltrent/stackoverflow_badge/issues).
- PRs [here](https://github.com/mattrltrent/stackoverflow_badge/pulls).
