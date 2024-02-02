async function fetchNews(sources, categories) {
  const posts = await Promise.all(
    sources
      .map(feed => `https://api.rss2json.com/v1/api.json?rss_url=${feed}`)
      .map(async url => {
        const res = await fetch(url)
        const data = await res.json()
        return data.items
      })
  )

  return posts
    .flatMap(posts => posts)
    .filter(post => categories.some(c => post.categories.includes(c)))
    .sort((a, b) => new Date(b.pubDate) - new Date(a.pubDate))
}
