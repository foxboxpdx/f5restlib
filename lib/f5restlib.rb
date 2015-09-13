# F5RestLib
# Implements a library for communication with the RESTful interface
# of an F5 BigIP device.
# @author Daniel Fox
require_relative 'f5restlib/version'

module F5RestLib
  def self.api
    @api
  end

  def self.configure(user, pass, url)
    @api = Her::API.new
    @api.setup :url => url do |c|
      c.use Faraday::BasicAuthentication.basic_auth(user, pass)
      c.use Faraday::Request::UrlEncoded
      c.use Her::Middleware::DefaultParseJSON
      c.use Faraday::Adapter::NetHttp
    end
  end
end

# Requirements
require 'her'
require 'yaml'
require 'optparse'
require 'net/scp'
require 'ipaddr'

# Models
require_relative 'f5restlib/virtual'
require_relative 'f5restlib/pool'
require_relative 'f5restlib/ssl'

# Helper classes, etc
require_relative 'f5restlib/help'
require_relative 'f5restlib/utils'
require_relative 'f5restlib/climain'
require_relative 'f5restlib/cliyaml'
require_relative 'f5restlib/cligetip'
require_relative 'f5restlib/clifetch'
require_relative 'f5restlib/clicert'
