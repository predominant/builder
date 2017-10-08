habitatConfig({
    habitat_api_url: "{{cfg.web.app_url}}",
    community_url: "{{cfg.web.community_url}}",
    docs_url: "{{cfg.web.docs_url}}",
    environment: "{{cfg.web.environment}}",
    friends_only: {{cfg.web.friends_only}},
    github_client_id: "{{cfg.github.client_id}}",
    github_api_url: "{{cfg.github.url}}",
    github_web_url: "{{cfg.github.web_url}}",
    github_app_url: "{{cfg.web.github_app_url}}",
    github_app_id: "{{cfg.github.app_id}}",
    source_code_url: "{{cfg.web.source_code_url}}",
    tutorials_url: "{{cfg.web.tutorials_url}}",
    version: "{{pkg.ident}}",
    www_url: "{{cfg.web.www_url}}",
    status_url: "{{cfg.web.status_url}}",
    forums_url: "{{cfg.web.forums_url}}",
    events_url: "{{cfg.web.events_url}}",
    roadmap_url: "{{cfg.web.roadmap_url}}",
    feature_requests_url: "{{cfg.web.feature_requests_url}}",
    slack_url: "{{cfg.web.slack_url}}",
    youtube_url: "{{cfg.web.youtube_url}}",
    demo_app_url: "{{cfg.web.demo_app_url}}",
    feature_flags: {{toJson cfg.web.feature_flags}}
});
