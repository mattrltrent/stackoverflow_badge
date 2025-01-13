# Stackoverflow Stats Badge For Your GitHub Profile 🚀

Example of it on my GitHub profile [here](https://github.com/mattrltrent). Star it if it helps you! ⭐️

1. **Full badge:**
 
   ![stackoverflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year)

3. **With `&mini=true` URL param appended:**
 
   ![stackoverflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year&mini=true)

## Adding full badge version

1. Go to [Stackoverflow](https://stackoverflow.com/), then click on your profile. Once on your profile, you should see a number in the URL bar. This is your **account ID**.

   For example, here is my account ID:

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_2.jpeg?raw=true"  style="display: inline"/>

2. Displayed on the stats badge is "*+ some number of reputation per some time period*". You need to choose over what time period you want us to calculate your reputation gain. The options are `day`,  `week`, `month`, `quarter`, and `year`. This is your **period**.

3. Take the base URL, and insert your newly found **account ID** and **period** where appropriate: 

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_3.jpg?raw=true"  style="display: inline"/>

   For example, here is the URL for my account if I wanted to show my reputation gain quarterly:

   <img  src="https://github.com/mattrltrent/stackoverflow_badge/blob/main/assets/github/demo_4.jpg?raw=true"  style="display: inline"/>

4. Here is the copyable version, just change the "USERNAME" and "PERIOD": https://stackoverflow-badge.herokuapp.com/stackoverflow?username=USERNAME&period=PERIOD.

5. Insert this URL from step 4 into your GitHub profile's `README.md` exactly like this (obviously with your own failure text and personalized URL):

   `![some text to render if the image fails to load](step 4 URL)`

6. If you're still struggling, check out how I added it to my own profile's `README.md` [here](https://github.com/mattrltrent/mattrltrent/blob/main/README.md?plain=1). It looks like: `![stackoverflow rep. badge](https://stackoverflow-badge.herokuapp.com/stackoverflow?username=13029516&period=year&mini=true)`, where this has my own "USERNAME" (13029516) and "PERIOD" (year).

## Adding mini badge version

This only shows the total reputation you have.

1. Do all the steps from the big badge above.

2. Append `&mini=true` to the URL (keep the other options, they just don't matter). Added via suggestion from [@yubinjodev](https://github.com/yubinjodev).

## Limitations

These restrictions shouldn't interfere with regular usage:

- The badge is cached for 15 minutes, so its data can possibly lag 15 minutes behind your true Stackoverflow stats. This is done to reduce server load.
- You're rate limited to loading 15 different stat badges every 60 seconds based on your IP.
- The StackAPI limits everyone to loading 300 different profiles every 24 hours based on IP.

## Feel free to create issues or PRs!

- Issues [here](https://github.com/mattrltrent/stackoverflow_badge/issues).
- PRs [here](https://github.com/mattrltrent/stackoverflow_badge/pulls).
