// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::toxml::ToXml;

/// Represents an enclosure in an RSS item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(feature = "builders", builder(setter(into), default))]
pub struct Enclosure {
    /// The URL of the enclosure.
    pub url: String,
    /// The length of the enclosure in bytes.
    pub length: String,
    /// The MIME type of the enclosure.
    pub mime_type: String,
}

impl Enclosure {
    /// Return the URL of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_url("http://example.com/audio.mp3");
    /// assert_eq!(enclosure.url(), "http://example.com/audio.mp3");
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Set the URL of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_url("http://example.com/audio.mp3");
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<String>,
    {
        self.url = url.into();
    }

    /// Return the content length of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_length("1000");
    /// assert_eq!(enclosure.length(), "1000");
    /// ```
    pub fn length(&self) -> &str {
        self.length.as_str()
    }

    /// Set the content length of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_length("1000");
    /// ```
    pub fn set_length<V>(&mut self, length: V)
    where
        V: Into<String>,
    {
        self.length = length.into();
    }

    /// Return the MIME type of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_mime_type("audio/mpeg");
    /// assert_eq!(enclosure.mime_type(), "audio/mpeg");
    /// ```
    pub fn mime_type(&self) -> &str {
        self.mime_type.as_str()
    }

    /// Set the MIME type of this enclosure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Enclosure;
    ///
    /// let mut enclosure = Enclosure::default();
    /// enclosure.set_mime_type("audio/mpeg");
    /// ```
    pub fn set_mime_type<V>(&mut self, mime_type: V)
    where
        V: Into<String>,
    {
        self.mime_type = mime_type.into();
    }
}

impl Enclosure {
    /// Builds an Enclosure from source XML
    pub fn from_xml<R: BufRead>(
        reader: &mut Reader<R>,
        mut atts: Attributes,
    ) -> Result<Self, Error> {
        let mut enclosure = Enclosure::default();

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                match attr.key {
                    b"url" => {
                        enclosure.url = attr.unescape_and_decode_value(reader)?;
                    }
                    b"length" => {
                        enclosure.length = attr.unescape_and_decode_value(reader)?;
                    }
                    b"type" => {
                        enclosure.mime_type = attr.unescape_and_decode_value(reader)?;
                    }
                    _ => {}
                }
            }
        }

        reader.read_to_end(b"enclosure", &mut Vec::new())?;

        Ok(enclosure)
    }
}

impl ToXml for Enclosure {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"enclosure";

        let mut element = BytesStart::borrowed(name, name.len());

        element.push_attribute(("url", self.url.as_str()));
        element.push_attribute(("length", self.length.as_str()));
        element.push_attribute(("type", self.mime_type.as_str()));

        writer.write_event(Event::Empty(element))?;
        Ok(())
    }
}
