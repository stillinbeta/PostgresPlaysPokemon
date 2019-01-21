.PHONY: proto

# Don't use PyPy
PYTHON=/usr/bin/python

proto:
	$(PYTHON) -m grpc_tools.protoc -I../proto --python_out=paas --grpc_python_out=paas ../proto/server.proto
