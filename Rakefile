require 'rake'

desc 'Preview the site with Jekyll'
task :preview do
    sh "bundle exec jekyll serve --watch --drafts --baseurl '' "
end

desc 'Search site and print specific deprecation warnings'
task :check do
    sh "bundle exec jekyll doctor"
end

desc  'create a draft post'
task :draft do
  require "time"
  t = Time.now
  title = ARGV[1].strip
  title_dc = title.gsub(/[^0-9a-zA-z]/,'-').downcase
  header = <<EOF
---
layout: post
title:  "#{title}"
date:  #{t}
categories:
- blog
- bioinformatics
permalink: #{title_dc}
#{ if ARGV[2] then "description: " + ARGV[2] end}
---

EOF
  File.open("_posts/#{t.strftime("%Y-%m-%d")}-#{title_dc}.md", "w").write(header)
  exit(0)
end
