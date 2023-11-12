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

```sh
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

## Actions Example

This workflow config will generate static API for Markdown documents located in the `./data` directory, and upload them to GitHub Pages.

```yaml
name: Deploy static API to Pages

on:
  push:
    branches: ['main']
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: 'pages'
  cancel-in-progress: true

jobs:
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Download md-indexer
        # WARNING: This will download the latest development build!
        run: wget "https://github.com/Zhousiru/md-indexer/releases/download/latest/md-indexer" && chmod +x md-indexer

      - name: Generate static API
        # Output to same path.
        run: ./md-indexer -i data -o data --size 20 --with-summary

      - name: Setup Pages
        uses: actions/configure-pages@v3

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v1
        with:
          path: './data'

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v1
```
