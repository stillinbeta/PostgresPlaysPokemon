.PHONY: proto

# Don't use PyPy
PYTHON=python

proto:
	$(PYTHON) -m grpc_tools.protoc -I../proto --python_out=paas --grpc_python_out=paas ../proto/server.proto
	sed -i -E 's/^import.*_pb2/from . \0/' paas/*.py

