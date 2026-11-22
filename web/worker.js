export default {
  fetch(request, env) {
    const url = new URL(request.url);
    if (url.hostname === "www.frankenrobotics.com") {
      url.hostname = "frankenrobotics.com";
      return Response.redirect(url.href, 308);
    }
    return env.ASSETS.fetch(request);
  },
};
