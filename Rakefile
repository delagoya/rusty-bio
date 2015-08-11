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
  title = ARGV[1].gsub(/[^0-9a-zA-z]/,'-').downcase
  `touch _drafts/#{title}.md`
  exit(0)
end