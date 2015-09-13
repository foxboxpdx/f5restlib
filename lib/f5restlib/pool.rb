# pool.rb
# Define a class that encapsulates an F5 Pool object
# @author FoxBoxPDX
#module F5RestLib
  class Pool
    include Her::Model
    uses_api F5RestLib::api
    attributes :name, :partition
    validates :name, presence: true
    validates :partition, presence: true
    collection_path "/mgmt/tm/ltm/pool"
    has_many :members
  end

  class Member
    include Her::Model
    uses_api F5RestLib::api
    attributes :name
    validates :name, presence: true
    collection_path "/mgmt/tm/ltm/pool/:pool_id/members"
  end
#end
