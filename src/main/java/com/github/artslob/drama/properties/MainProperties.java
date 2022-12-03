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
}
