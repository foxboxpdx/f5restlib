# F5RestLib
# Implements a library for communication with the RESTful interface
# of an F5 BigIP device.
# @author FoxBoxPDX
require_relative 'f5restlib/version'

# Requirements
require 'her'
require 'yaml'
require 'optparse'
require 'net/scp'
require 'ipaddr'
require_relative 'f5restlib/f5jsonparser'

module F5RestLib
  def self.api
    @api
  end

  def self.configure(user, pass, url)
    @api = Her::API.new
    @api.setup :url => url, :ssl => {:verify => false } do |c|
      c.use Faraday::Request::BasicAuthentication, user, pass
      c.use Faraday::Request::UrlEncoded
      #c.use Her::Middleware::DefaultParseJSON
      c.use F5JSONParser
      c.use Faraday::Adapter::NetHttp
    end
  end

  def self.load_stuff
    # Include the models after configuration has occurred.
    require_relative 'f5restlib/virtual'
    require_relative 'f5restlib/pool'
    require_relative 'f5restlib/ssl'
  end
end

F5RestLib.configure

# Helper classes, etc
require_relative 'f5restlib/help'
require_relative 'f5restlib/utils'
require_relative 'f5restlib/climain'
require_relative 'f5restlib/cliyaml'
require_relative 'f5restlib/cligetip'
require_relative 'f5restlib/clifetch'
require_relative 'f5restlib/clicert'
