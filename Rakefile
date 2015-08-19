require 'rake'

desc 'Preview the site with Jekyll'
task :preview do
    sh "jekyll serve --watch --drafts --baseurl '' "
end

desc 'Search site and print specific deprecation warnings'
task :check do 
    sh "jekyll doctor"
end

desc  'create a draft post'
task :draft do 
  require "time"
  title = ARGV[1].strip
  title_dc = title.gsub(/[^0-9a-zA-z]/,'-').downcase
  header = <<EOF
---
layout: post
title:  "#{title}"
date:  #{Time.now}
categories:
- blog
permalink: #{title_dc}
#{ if ARGV[2] then "description: " + ARGV[2] end}
---
EOF
  File.open("_drafts/#{title_dc}.md", "w").write(header)
  exit(0)
end

