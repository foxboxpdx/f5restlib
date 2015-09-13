# ssl.rb
# Define classes to encapsulate the F5 SSL objects
# @author FoxBoxPDX
#module F5RestLib
  class ClientSSLProfile
    include Her::Model
    uses_api F5RestLib::api
    attributes :name, :partition, :cert, :key
    validates :name, presence: true
    validates :partition, presence: true
    validates :cert, presence: true
    validates :key, presence: true
    collection_path "/mgmt/tm/ltm/profile/client-ssl"
  end

  class Key
    include Her::Model
    uses_api F5RestLib::api
    attributes :name, :partition
    validates :name, presence: true
    validates :partition, presence: true
    collection_path "/mgmt/tm/sys/crypto/key"
  end

  class Cert
    include Her::Model
    uses_api F5RestLib::api
    attributes :name, :partition
    validates :name, presence: true
    validates :partition, presence: true
    collection_path "/mgmt/tm/sys/crypto/cert"
  end
#end
