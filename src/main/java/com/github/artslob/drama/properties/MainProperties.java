package com.github.artslob.drama.properties;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Configuration;

@Configuration
public class MainProperties {

    @Value("${reddit_app_id}")
    public String reddit_app_id;

    @Value("${reddit_app_password}")
    public String reddit_app_password;

    @Value("${redirect_uri}")
    public String redirect_uri;

    @Value("${access_token_url}")
    public String access_token_url;

    @Value("${authorize_url}")
    public String authorize_url;

    @Value("${api_url}")
    public String api_url;
}
