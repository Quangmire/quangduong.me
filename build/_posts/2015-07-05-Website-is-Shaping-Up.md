---
layout: post
title: Website is Shaping Up
date: 2015-07-05 22:03:00
summary: Archives and Tag system up and running.
tags: tutorial website jekyll
---
After browsing the Internet (recommended way for people as inexperienced as yours truly to learn) for some time,
I decided to implement tags and an archive to hold all my posts. After toiling endlessly, witness the fruit of my
labor.

*-takes a deep breath-*

##Tag System
The tagging system is taken straight from this [blog post](http://charliepark.org/tags-in-jekyll/) and is very simple
to install. All you have to do is create a `_plugins` folder in the root of your [Jekyll](http://jekyllrb.com)
directory. Inside it place a ruby file with the source code provided in the blog post. In addition, you have to create
a file inside the `_layouts` directory called `tag_index.html` and fill it with the html and css necessary to correctly
render a page that will hold all the posts of a certain tag.

##Archives
Using Jekyll's built in loops, I looped through all the posts and embedded them all on the page.
<div class="code">
{% highlight jinja %}
{% raw %}
	{% for post in site.posts %}  
	// Insert content here. Mainly the 'post.title', 'post.content',  
	// and 'post.summary' variables.  
{% endfor %}
{% endraw %}
{% endhighlight %}
</div>
For the `tag_index.html` file, I used the same format except I additionally checked if the tag matched for every post.
<div class="code">
{% highlight jinja %}
{% raw %}
	{% for tag in post.tags %}  
	{% if tag == page.tag %}
	// Insert content here. Mainly the 'post.title', 'post.content',  
	// and 'post.summary' variables.  
	{% endif %}
{% endfor %}
{% endraw %}
{% endhighlight %}
</div>