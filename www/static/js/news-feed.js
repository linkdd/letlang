const fetch_articles = async (authors, categories) => {
  const promises = authors
    .map(author => `https://medium.com/feed/@${author}`)
    .map(feed => `https://api.rss2json.com/v1/api.json?rss_url=${feed}`)
    .map(async url => {
      const res = await fetch(url)
      const data = await res.json()
      return data.items.filter(post => {
        if (post.categories.length > 0) {
          for (const category of categories) {
            if (post.categories.indexOf(category) !== -1) {
              return true
            }
          }
        }

        return false
      })
    })

  const posts = await Promise.all(promises)
  return posts
    .flatMap(postList => postList)
    .sort((item, other) => {
      if (item.pubDate < other.pubDate) {
        return 1
      }
      else if (item.pubDate > other.pubDate) {
        return -1
      }
      return 0
    })
}

const render_articles = (articles, div) => {
  for (const article of articles) {
    const column = $('<div/>', { class: 'column is-one-quarter' })
    const card = $('<div/>', { class: 'card is-full-height is-flex is-flex-direction-column' }).append(
      $('<div/>', { class: 'card-image' }).append(
        $('<figure/>', { class: 'image' }).append(
          $('<img/>', { src: article.thumbnail, alt: 'Thumbnail' })
        )
      ),
      $('<div/>', { class: 'card-content' }).append(
        $('<div/>', { class: 'content' }).append(
          $('<p/>', { class: 'title is-size-4' }).append(
            $('<a/>', { text: article.title, href: article.link, target: '_blank' })
          ),
          $('<p/>', { class: 'subtitle is-size-6', text: `@${article.author}` })
        )
      ),
      $('<div/>', { class: 'card-footer', style: 'margin-top: auto;' }).append(
        $('<div/>', { class: 'container p-3' }).append(
          $('<p/>', { class: 'has-text-centered' }).append(
            ...article.categories.map(category =>
              $('<span/>', { class: 'tag is-primary mx-1', text: category })
            )
          ),
          $('<p/>', {
            class: 'is-size-7 is-uppercase has-text-grey has-text-weight-semibold has-text-right mt-3',
            text: moment(article.pubDate).format('dddd, MMMM Do, YYYY')
          })
        )
      )
    )

    column.append(card)
    div.append(column)
  }
}

$(() => {
  const articlesDiv = $('#articles')
  const authors = articlesDiv.data('authors')
  const categories = articlesDiv.data('categories')
  fetch_articles(authors, categories).then(
    articles => render_articles(articles, articlesDiv),
    console.error
  )
})