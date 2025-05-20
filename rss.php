<?php require_once( 'editar/cms.php' ); ?>
<cms:content_type 'text/xml' /><cms:concat '<' '?xml version="1.0" encoding="' k_site_charset '"?' '>' />
<cms:template title='RSS' hidden='1' order='999999999'  /> 
<rss version="2.0">
  <channel>
    <title>Blog de Díaz&Pons</title>
    <description>Noticias y avatares</description>
    <language>es</language>
    <pubDate><cms:date format='D, d M Y H:i:s' gmt='1'/> GMT</pubDate>
    <generator>Couch CMS</generator>

    <cms:pages masterpage='blog.php' limit="10">
    <item>
        <title><cms:show k_page_title /></title>
        <link><cms:show k_page_link /></link>
        <!--<description>
            <cms:html_encode>
                <cms:excerptHTML><cms:show my_news_text /></cms:excerptHTML>
            </cms:html_encode>
        </description>-->

        <pubDate><cms:date k_page_date format='D, d M Y H:i:s' gmt='1'/> GMT</pubDate>
    </item>
    </cms:pages>
    
    <cms:pages masterpage='noticias.php' limit="10">
    <item>
        <title><cms:show k_page_title /></title>
        <link><cms:show k_page_link /></link>
        <!--<description>
            <cms:html_encode>
                <cms:excerptHTML><cms:show my_news_text /></cms:excerptHTML>
            </cms:html_encode>
        </description>-->

        <pubDate><cms:date k_page_date format='D, d M Y H:i:s' gmt='1'/> GMT</pubDate>
    </item>
    </cms:pages>

 </channel>
</rss>
<?php COUCH::invoke(); ?>