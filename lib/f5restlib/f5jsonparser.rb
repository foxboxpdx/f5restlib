# F5JSONParser
# Define a custom json parser to handle incoming messages
# from the F5
# @author FoxBoxPDX
class F5JSONParser < Faraday::Response::Middleware
  def on_complete(env)
    # Set env[:body] to a hash with three symbol keys:
    # :data, :errors, and :metadata.
    json = MultiJson.load(env[:body], symbolize_keys: true)
    env[:body] = {
      data: json[:items],
      errors: json[:errors],
      metadata: json[:metadata]
    }
  end
end
