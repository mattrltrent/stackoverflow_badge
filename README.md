# Stack Overflow Stats Badge For Your GitHub Profile 🚀

<img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_1.png?raw=true"  style="display: inline"/>

Example of it on my GitHub profile [here](https://github.com/mattrltrent).

## Simple to add:

1. Go to [Stack Overflow](https://stackoverflow.com/), then click on your profile. Once on your profile, you should see a number in the URL bar. This is your **account ID**.

   For example, here is my account ID:

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_2.jpeg?raw=true"  style="display: inline"/>

2. Displayed on the stats badge is "*+ some number of reputation per some time period*". You need to choose over what time period you want us to calculate your reputation gain. The options are `day`,  `week`, `month`, `quarter`, and `year`. This is your **period**.

3. Take the base URL, and insert your newly found **account ID** and **period** where appropriate: 

   URL: <code>https://stackoverflow-badge.herokuapp.com/stack_overflow?username=<b>account ID</b>&period=<b>period</b></code>

   For example, here is the URL for my account if I wanted to show my reputation gain quarterly: <code>https://stackoverflow-badge.herokuapp.com/stack_overflow?username=<b>13029516</b>&period=<b>quarter</b></code>

4. Insert this into your GitHub profile's `README.md`, obviously with your own failure text and personalized URL:

   `![some text to render if the image fails to load](YOUR URL)`

## Limitations:

These restrictions shouldn't interfere with regular usage:

- You're rate limited to loading 15 different stat badges every 60 seconds based on your IP.
- The StackAPI limits everyone to loading 300 different profiles every 24 hours based on IP.


## Feel free to create issues or PRs!

- Issues [here](https://github.com/mattrltrent/stackoverflow_badge/issues).
- PRs [here](https://github.com/mattrltrent/stackoverflow_badge/pulls).