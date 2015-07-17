-- wrk.method = "POST"
-- wrk.body = "foo"

-- example dynamic request script which demonstrates changing
-- the request path and a header for each request
-------------------------------------------------------------
-- NOTE: each wrk thread has an independent Lua scripting
-- context and thus there will be one counter per thread

counter = 0

request = function()
   path = "/thread/0/page/" .. counter
   counter = counter + 1
   counter = counter % 65536
   return wrk.format(nil, path)
end
