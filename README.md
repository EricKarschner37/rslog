# rslog

We need another blog platform, right?

## Overview

Blog tooling, written in rust (so it's blazingly fast and what have you). Right now, my goal is just to make something that works for _me_. I don't wanna start a podcast, so this is the next best way for me to put my opinions out into the world.

If this goes well, eventually I may want to genericize it so that anyone can deploy their own instance. That's always fun. Not the main goal though!

## Design decisions

- I'm writing this in Rust, cause I didn't wanna make another flask app
- I'm going to avoid using javascript as much as possible. I write enough of that at work! Plus, I'm always seeing great demos of what's possible with the latest and greatest in HTML + CSS -- I wanna explore that!
- I will not be using AI to write any code or content for my blog. Again, I do enough of that at work. This is for fun.
- By and large, I'm going to attempt to err on the side of not using libraries, and making things myself. This means it will probably end up being bad, and slow, and ugly. I don't care! I want to learn.
  - One immediate exception: I will be using [rocket](https://rocket.rs) to handle the web server. I've implemented plenty of http servers already in my life

## TODOs

### Tagging system

I intend on writing about a number of disparate topics, from AI to urban planning to my homelab to whatever book I happen to be reading at the moment. It'll be important to have a featureful tagging system so that "readers" (currently, and likely permanently, theoretical) can choose to only view (and/or subscribe to) what they choose.

### Subscriptions

#### Email

Might need to set up SMTP? I know that can be a hassle with getting other providers to trust me though, so we'll see. Probably an easy way to just proxy it to gmail or whatever

#### RSS

I love RSS! My RSS feed is currently my main "scrolling" app, and it's certainly been better for my brain than reddit.

Plus, I've had an idea for a bit of republishing periodicals (currently reading through all the original Sherlock Holmes stories) over a weekly RSS feed, so this tech would likely be reused.

### Auth

Goes without saying. I need to be able to upload stuff. I also need nobody else to be able to upload stuff.

This is one of those things I'm going to (try to) build myself.
