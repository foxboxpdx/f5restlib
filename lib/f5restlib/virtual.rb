# virtual.rb
# Define a class that encapsulates an F5 Virtual (VIP) object
# @author FoxBoxPDX
#module F5RestLib
  class Virtual
    include Her::Model
    uses_api F5RestLib::api
    attributes :name, :partition, :pool
    validates :name, presence: true
    validates :partition, presence: true
    validates :pool, presence: true
    collection_path "/mgmt/tm/ltm/virtual"
  end
#end
