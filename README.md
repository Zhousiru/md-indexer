# md-indexer

An opinionated static API generator for Markdown documents.

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
```

## Usage

```
md-indexer [OPTIONS] --input <INPUT> --output <OUTPUT> --size <SIZE>
```

```
Options:
  -i, --input <INPUT>    The input base path of documents
  -o, --output <OUTPUT>  The output base path of index files
      --size <SIZE>      Maximum number of documents in a single page file
      --with-content     Include document content in index files
      --with-summary     Include document summary (marked with `<!--more-->`) in index files
  -h, --help             Print help
```

## Predefined Frontmatter

### `date`

Date string in RFC3339.

Documents will be sorted by date in descending order. If `date` does not exist, the document will be pinned.
