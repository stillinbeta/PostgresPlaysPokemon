.PHONY: proto

# Don't use PyPy
PYTHON=/usr/bin/python

proto:
	$(PYTHON) -m grpc_tools.protoc -I../proto --python_out=. --grpc_python_out=. ../proto/server.proto
