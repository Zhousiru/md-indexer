# md-indexer

An opinionated static API generator for Markdown documents

**From**

```
site
  home.md
  posts
    post-1.md
    post-2.md
    ...
  news
    news-1.md
    news-2.md
    ...
```

**To**

```
index
  page-1.json   <- Index page of `home.md`, `post-x.md` and `news-x.md`
  posts
    page-1.json <- Index page of `post-x.md`
    page-2.json
    ...
  news
    page-1.json <- Index page of `news-x.md`
    page-2.json
    ...
...
```
