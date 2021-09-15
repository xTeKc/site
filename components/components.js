Vue.component('blog-list', { props: ['limit'], data: function(){  return {   articles: null  } },

created: function(){  var query = deliveryClient   .items()   .type('blog_post')   .elementsParameter(['link', 'title', 'image_url', 'image', 'teaser'])   .orderParameter('elements.published', SortOrder.desc);   if (this.limit){   query = query.limitParameter(this.limit);  }  query   .getPromise()   .then(response =>    this.$data.articles = response.items.map(item => ({     url: item.link.value,     header: item.title.value,     image: item.image_url.value != '' ? item.image_url.value : item.image.assets[0].url,     teaser: item.teaser.value    }))   ); },

